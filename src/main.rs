mod renderer;
use std::io::Write;

use renderer::*;

const SHOW_FFMPEG_OUTPUT: bool = false;

fn main() {
    let clock = std::time::Instant::now();
    let mut renderer = Renderer::new(
        8,            // duration in seconds
        60,           // frames per second
        1800,         // width
        1000,         // height
        "output.mp4", // output file
    );

    let title = "A SIMPLE TITLE";
    let title_font_size = 72.0;

    // temporary frame for getting the context

    let title_dim = graphics::measure_text(
        &renderer.get_temporary_context(),
        title,
        title_font_size,
        None,
    );

    let (width, height) = renderer.get_frame_size();
    let (width, height) = (width as f64, height as f64);

    // calculate the position of the title
    let center_x = width / 2.0;
    let center_y = height / 2.0;

    let title_x = center_x - title_dim.0 / 2.0;
    let title_y = center_y - title_dim.1 / 2.0;

    let mut title_reveal_animation = Animator::new(
        renderer.get_duration_parameter(1.0),
        renderer.get_duration_parameter(6.0),
        ezing::cubic_inout,
        FinishAction::RepeatEnd,
    );

    while let Some(frame) = renderer.get_frame() {
        let cr = frame.get_context();

        cr.set_antialias(cairo::Antialias::Best);

        // clear the background
        graphics::clear(cr, graphics::color::BLACK);

        title_reveal_animation.draw(renderer.t(), |t| {
            graphics::draw_text(
                cr,
    
                title,
                (title_x, title_y),
                title_font_size,
                graphics::color::WHITE,
                None,
            );
    
            graphics::draw_rectangle(
                cr,
                (title_x + title_dim.0 * t, title_y),
                title_dim.0,
                title_dim.1,
                1.0,
                graphics::color::BLACK,
                Some(graphics::color::BLACK),
            );
        });

        // submit the frame to the renderer (blocking)
        renderer.submit(frame);

        print!(
            "\rFrame: {}/{}",
            renderer.frame_count(),
            renderer.total_frame_count()
        );
        std::io::stdout().flush().unwrap();
    }

    let render_time = clock.elapsed();

    renderer.finish();
    println!(
        "\rFinished              \n    avg. frame time: {:.2?}\n    total time: {:.2?}",
        render_time / renderer.total_frame_count(),
        clock.elapsed()
    );
}
