mod renderer;
use std::io::Write;

use renderer::*;

const SHOW_FFMPEG_OUTPUT: bool = false;

fn main() {
    let clock = std::time::Instant::now();
    let mut renderer = Renderer::new(
        7,            // duration in seconds
        24,           // frames per second
        1800,         // width
        1000,         // height
        "output.mp4", // output file
    );

    let mut animation = Animator::new(
        renderer.get_duration_parameter(1.0),
        renderer.get_duration_parameter(5.0),
        ezing::cubic_inout,
        FinishAction::RepeatEnd,
    );

    while let Some(frame) = renderer.get_frame() {
        let cr = frame.get_context();

        cr.set_antialias(cairo::Antialias::Best);

        // clear the background
        graphics::clear(cr, graphics::Color::Black);

        let (width, height) = renderer.get_frame_size();
        let spacing = 50.0;
        draw_coordinate_axes(cr, width as _, height as _);
        draw_grid_lines(cr, width as _, height as _, spacing, graphics::Color::Gray);

        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        graphics::draw_text(
            cr,
            "x",
            width as f64 - 60.0,
            center_y - 60.0,
            32.0,
            graphics::Color::White,
        );

        graphics::draw_text(
            cr,
            "y",
            center_x + 30.0,
            10.0,
            32.0,
            graphics::Color::White,
        );

        // draw a point which is initially at origin and then moves to (4, 3)
        animation.draw(renderer.t(), |t| {
            let x = center_x + 4.0 * spacing * t;
            let y = center_y - 3.0 * spacing * t;

            graphics::draw_circle(
                cr,
                x,
                y,
                5.0,
                10.0,
                graphics::Color::Yellow,
                Some(graphics::Color::Yellow),
            );

            if t > 0.05 {
                let text = format!(
                    "({:.2}, {:.2})",
                    (x - center_x) / 50.0,
                    -(y - center_y) / 50.0
                );
                graphics::draw_text(cr, &text, x + 10.0, y - 30.0, 32.0, graphics::Color::White);
            }
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

    renderer.finish();
    println!("Finished, took: {:.2?}", clock.elapsed());
}

fn draw_coordinate_axes(cr: &cairo::Context, width: f64, height: f64) {
    let center_x = width / 2.0;
    let center_y = height / 2.0;

    // Draw x-axis
    graphics::draw_line(
        cr,
        0.0,
        center_y + 1.0,
        width,
        center_y + 1.0,
        2.0,
        graphics::Color::White,
    );

    // Draw y-axis
    graphics::draw_line(
        cr,
        center_x,
        0.0,
        center_x,
        height,
        2.0,
        graphics::Color::White,
    );
}

fn draw_grid_lines(
    cr: &cairo::Context,
    width: f64,
    height: f64,
    spacing: f64,
    color: graphics::Color,
) {
    let (r, g, b, a) = color.to_rgba();
    cr.set_source_rgba(r, g, b, a);

    // Draw vertical lines
    for x in (0..(width as usize)).step_by(spacing as usize) {
        graphics::draw_line(cr, x as f64, 0.0, x as f64, height, 1.0, color);
    }

    // Draw horizontal lines
    for y in (0..(height as usize)).step_by(spacing as usize) {
        graphics::draw_line(cr, 0.0, y as f64, width, y as f64, 1.0, color);
    }
}
