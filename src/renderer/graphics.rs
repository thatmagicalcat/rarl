use cairo::{Context, Format, ImageSurface};

use pango::FontDescription;
use pangocairo::functions::{create_layout, show_layout};

const DEFAULT_FONT: &str = "JetBrainsMono";

pub type Color = [f64; 4];

pub mod color {
    //! RGBA values
    //! Note: Put the color values in BGRA format i you're using
    //! `set_source_rgba` function from cairo

    use super::Color;

    pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
    pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
    pub const GRAY: Color = [0.5, 0.5, 0.5, 1.0];
    pub const ORANGE: Color = [1.0, 0.271, 0.0, 1.0];
    pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
    pub const TRANSPARENT: Color = [0.0, 0.0, 0.0, 0.0];
}

pub fn clear(cr: &Context, color: Color) {
    let [b, g, r, a] = color;
    cr.set_source_rgba(r, g, b, a);
    cr.paint().unwrap();
}

pub fn measure_text(cr: &Context, text: &str, size: f64, font_name: Option<&str>) -> (f64, f64) {
    // Create Pango layout
    let layout = create_layout(cr);

    // Set font
    let font_desc =
        FontDescription::from_string(&format!("{} {}", font_name.unwrap_or(DEFAULT_FONT), size));
    layout.set_font_description(Some(&font_desc));

    // Set text (could include markup)
    layout.set_markup(text);

    // Get the size of the text
    let (width, height) = layout.pixel_size();
    (width as f64, height as f64)
}

pub fn draw_text(
    cr: &Context,
    text: &str,
    (x, y): (f64, f64),
    size: f64,
    text_color: Color,
    font_name: Option<&str>,
) {
    cr.new_path();

    let [b, g, r, a] = text_color;
    cr.set_source_rgba(r, g, b, a);

    cr.move_to(x, y);

    // Create Pango layout
    let layout = create_layout(cr);

    // Set font
    let font_desc =
        FontDescription::from_string(&format!("{} {}", font_name.unwrap_or(DEFAULT_FONT), size));
    layout.set_font_description(Some(&font_desc));

    // Set text (could include markup)
    layout.set_markup(text);

    // Render text
    show_layout(cr, &layout);
}

pub fn draw_line(
    cr: &Context,
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
    thickness: f64,
    color: Color,
) {
    cr.new_path();

    let [b, g, r, a] = color;
    cr.set_source_rgba(r, g, b, a);
    cr.move_to(x1, y1);
    cr.line_to(x2, y2);
    cr.set_line_width(thickness);
    cr.stroke().unwrap();
}

pub fn draw_rectangle(
    cr: &Context,
    (x, y): (f64, f64),
    width: f64,
    height: f64,
    thickness: f64,
    border_color: Color,
    fill_color: Option<Color>,
) {
    cr.new_path();

    let [b, g, r, a] = border_color;
    cr.set_source_rgba(r, g, b, a);
    cr.rectangle(x, y, width, height);
    cr.set_line_width(thickness);
    cr.stroke_preserve().unwrap();

    if let Some(fill_color) = fill_color {
        let [b, g, r, a] = fill_color;
        cr.set_source_rgba(r, g, b, a);
        cr.fill().unwrap();
    }
}

pub fn draw_circle(
    cr: &Context,
    (center_x, center_y): (f64, f64),
    radius: f64,
    thickness: f64,
    color: Color,
    fill_color: Option<Color>,
) {
    cr.new_path();

    let [b, g, r, a] = color;
    cr.set_source_rgba(r, g, b, a);
    cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
    cr.set_line_width(thickness);
    cr.stroke_preserve().unwrap();

    // fill color
    if let Some(fill_color) = fill_color {
        let [b, g, r, a] = fill_color;
        cr.set_source_rgba(r, g, b, a);
        cr.fill().unwrap();
    }
}

pub fn draw_arc(
    cr: &Context,
    (x, y): (f64, f64),
    angle1: f64,
    angle2: f64,
    radius: f64,
    thickness: f64,
    color: Color,
) {
    cr.new_path();

    let [b, g, r, a] = color;
    cr.set_source_rgba(r, g, b, a);
    cr.arc(x, y, radius, angle1, angle2);
    cr.set_line_width(thickness);
    cr.stroke().unwrap();
}
