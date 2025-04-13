use std::time::Instant;

mod renderer;
use renderer::Renderer;

const SHOW_FFMPEG_OUTPUT: bool = false;

fn main() {
    let clock = Instant::now();
    let mut renderer = Renderer::new(5, 1, 1920, 1080, "output.mp4");

    // Maxwell's equations
    let typst_code = r#"
        #show math.equation: eq => [
          #text(fill: white, [ #eq ])
        ]

        $ nabla dot arrow(E) = rho / epsilon_0 $
        $ nabla dot arrow(B) = 0 $
        $ nabla times arrow(E) = - (partial arrow(B)) / (partial t) $
        $ nabla times arrow(B) = mu_0(arrow(J) + epsilon_0 (partial arrow(E)) / (partial t)) $
    "#;

    // pre-render SVG
    let mut eqn = renderer.render_typst(typst_code);
    eqn.translate(100.0, 100.0).scale(3.0, 3.0).build();

    while let Some(frame) = renderer.get_frame() {
        let cr = frame.get_context();

        // black background
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.paint().unwrap();

        cr.set_source_rgb(1.0, 1.0, 1.0);
        frame.draw_text("Hello, World!", 700.0, 200.0, 44.0);

        eqn.render(&frame);

        renderer.submit(frame);
    }

    renderer.finish();
    println!("\rFinished, took: {:.2?}\t\t", clock.elapsed());
}
