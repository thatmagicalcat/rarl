use std::io::{BufWriter, Write};
use std::process;

use cairo::{Format, ImageSurface};

use crate::SHOW_FFMPEG_OUTPUT;

mod frame;
pub use frame::Frame;

pub struct Renderer {
    surface: cairo::ImageSurface,

    duration_secs: f64,
    frame_counter: f64,
    total_frame_count: f64,
    fps: f64,

    ffmpeg_process: process::Child,
    ffmpeg_stdin: Option<BufWriter<process::ChildStdin>>,
}

impl Renderer {
    pub fn new(duration_secs: u32, fps: u32, width: u32, height: u32, output_path: &str) -> Self {
        let surface = ImageSurface::create(Format::ARgb32, width as _, height as _)
            .expect("Can't create surface");

        let fps_text = fps.to_string();

        #[rustfmt::skip]
        let args = [
            "-y", // overwrite output without asking
            "-f", "rawvideo", // input format is raw video
            "-pixel_format", "rgba",
            "-r", fps_text.as_str(),
            "-video_size", &format!("{}x{}", width, height),
            "-i", "-", // input comes from a pipe
            "-vcodec", "libx264",
            "-pix_fmt", "yuv420p",
            output_path,
        ];

        let mut ffmpeg_command = process::Command::new("ffmpeg");
        ffmpeg_command.args(args).stdin(process::Stdio::piped());

        if !SHOW_FFMPEG_OUTPUT {
            ffmpeg_command
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null());
        }

        let mut ffmpeg_process = ffmpeg_command.spawn().unwrap();

        let ffmpeg_stdin = Some(BufWriter::new(
            ffmpeg_process
                .stdin
                .take()
                .expect("failed to open child stdin"),
        ));

        Self {
            surface,
            ffmpeg_process,
            ffmpeg_stdin,

            frame_counter: 0.0,
            duration_secs: duration_secs as _,
            fps: fps as _,
            total_frame_count: duration_secs as f64 * fps as f64,
        }
    }

    pub fn get_frame(&mut self) -> Option<Frame> {
        (self.frame_counter < self.total_frame_count).then(|| Frame::new(&self.surface))
    }

    pub fn submit(&mut self, mut frame: Frame) {
        drop(frame.context.take());

        let data = self.surface.data().unwrap();
        self.ffmpeg_stdin
            .as_mut()
            .unwrap()
            .write_all(&data)
            .unwrap();

        self.frame_counter += 1.0;
    }

    /// Returns true of video was successfully rendered
    pub fn finish(&mut self) -> bool {
        assert_eq!(self.frame_counter, self.total_frame_count);

        drop(self.ffmpeg_stdin.take());
        let status = self.ffmpeg_process.wait().unwrap();
        status.success()
    }

    pub fn t(&self) -> f64 {
        self.frame_counter / self.total_frame_count
    }

    pub fn get_duration_parameter(&self, duration_secs: f64) -> f64 {
        assert!(self.duration_secs >= duration_secs);

        duration_secs * self.fps / self.total_frame_count
    }

    pub fn total_frame_count(&self) -> u32 {
        self.total_frame_count as _
    }

    /// Returns the number of rendered frame
    pub fn frame_count(&self) -> u32 {
        self.frame_counter as _
    }

    pub fn render_typst(&self, typst_code: &str) -> Typst {
        const PREAMBLE: &str = "#set page(fill: none, height: auto, width: auto, margin: 0pt)";

        let mut typst_process = std::process::Command::new("typst")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .arg("compile")
            .arg("-")
            .arg("--format")
            .arg("svg")
            .arg("--pages")
            .arg("1")
            .arg("-")
            .spawn()
            .expect("failed to spawn `typst` process");

        writeln!(
            BufWriter::new(typst_process.stdin.take().expect("failed to open stdin")),
            "{PREAMBLE}\n{typst_code}"
        )
        .expect("failed to write to typst stdin");

        let output = typst_process
            .wait_with_output()
            .expect("failed to wait for typst process to finish");

        assert!(output.status.success());

        std::fs::write("image.svg", &output.stdout).unwrap();

        let rtree =
            resvg::usvg::Tree::from_data(&output.stdout, &resvg::usvg::Options::default()).unwrap();

        Typst::new(rtree)
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        if self.ffmpeg_stdin.is_some() {
            self.finish();
        }
    }
}

pub struct Typst {
    tree: resvg::usvg::Tree,
    surface: Option<ImageSurface>,

    translation: (f32, f32),
    scale: (f32, f32),
}

impl Typst {
    fn new(tree: resvg::usvg::Tree) -> Self {
        Self {
            tree,
            surface: None,

            translation: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }

    fn get_transform(&self) -> resvg::usvg::Transform {
        // translation is provided in the cairo set source surface call
        resvg::usvg::Transform::identity().pre_scale(self.scale.0, self.scale.1)
    }

    #[doc(alias = "rebuild")]
    pub fn build(&mut self) {
        let size = self.get_size();

        let width = (size.width() * self.scale.0) as u32;
        let height = (size.height() * self.scale.1) as u32;

        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(width, height).expect("Failed to create pixmap");

        resvg::render(&self.tree, self.get_transform(), &mut pixmap.as_mut());

        self.surface = Some(
            ImageSurface::create_for_data(
                pixmap.data().to_vec(),
                Format::ARgb32,
                width as _,
                height as _,
                width as i32 * 4,
            )
            .expect("Couldn't create image surface"),
        );
    }

    pub fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        self.translation = (x, y);
        self
    }

    pub fn scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        self.scale = (sx, sy);
        self
    }

    pub fn get_size(&self) -> resvg::usvg::Size {
        self.tree.size()
    }

    pub fn render(&self, target: &Frame) {
        assert!(self.surface.is_some());

        let cr = target.get_context();

        cr.set_source_surface(
            self.surface.as_ref().unwrap(),
            self.translation.0 as _,
            self.translation.1 as _,
        )
        .unwrap();

        cr.paint().unwrap();
    }
}
