use cairo::{Context, Format, ImageSurface};

use pango::FontDescription;
use pangocairo::functions::{create_layout, show_layout};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
    Gray,
    Yellow,
    Black,
    White,
    Rgb(f64, f64, f64),
    Rgba(f64, f64, f64, f64),
}

impl Color {
    pub fn to_rgba(self) -> (f64, f64, f64, f64) {
        match self {
            Color::Red => (1.0, 0.0, 0.0, 1.0),
            Color::Green => (0.0, 1.0, 0.0, 1.0),
            Color::Gray => (0.5, 0.5, 0.5, 1.0),
            Color::Blue => (0.0, 0.0, 1.0, 1.0),
            Color::Yellow => (0.0, 1.0, 1.0, 1.0),
            Color::Black => (0.0, 0.0, 0.0, 1.0),
            Color::White => (1.0, 1.0, 1.0, 1.0),
            Color::Rgb(r, g, b) => (r, g, b, 1.0), // Default alpha to 1.0
            Color::Rgba(r, g, b, a) => (r, g, b, a),
        }
    }
}

pub fn clear(cr: &Context, color: Color) {
    let (r, g, b, a) = color.to_rgba();
    cr.set_source_rgba(r, g, b, a);
    cr.paint().unwrap();
}

pub fn draw_text(cr: &Context, text: &str, x: f64, y: f64, size: f64, text_color: Color) {
    cr.new_path();

    let (r, g, b, a) = text_color.to_rgba();
    cr.set_source_rgba(r, g, b, a);

    cr.move_to(x, y);

    // Create Pango layout
    let layout = create_layout(cr);

    // Set font
    let font_desc = FontDescription::from_string(&format!("JetBrainsMono {}", size));
    layout.set_font_description(Some(&font_desc));

    // Set text (could include markup)
    layout.set_markup(text);

    // Render text
    show_layout(cr, &layout);
}

pub fn draw_line(cr: &Context, x1: f64, y1: f64, x2: f64, y2: f64, thickness: f64, color: Color) {
    cr.new_path();

    let (r, g, b, a) = color.to_rgba();
    cr.set_source_rgba(r, g, b, a);
    cr.move_to(x1, y1);
    cr.line_to(x2, y2);
    cr.set_line_width(thickness);
    cr.stroke().unwrap();
}

pub fn draw_rectangle(
    cr: &Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    thickness: f64,
    color: Color,
) {
    cr.new_path();

    let (r, g, b, a) = color.to_rgba();
    cr.set_source_rgba(r, g, b, a);
    cr.rectangle(x, y, width, height);
    cr.set_line_width(thickness);
    cr.stroke().unwrap();
}

pub fn draw_circle(
    cr: &Context,
    x: f64,
    y: f64,
    radius: f64,
    thickness: f64,
    color: Color,
    fill_color: Option<Color>,
) {
    cr.new_path();

    let (r, g, b, a) = color.to_rgba();
    cr.set_source_rgba(r, g, b, a);
    cr.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
    cr.set_line_width(thickness);
    cr.stroke_preserve().unwrap();

    // fill color
    if let Some(fill_color) = fill_color {
        let (r, g, b, a) = fill_color.to_rgba();
        cr.set_source_rgba(r, g, b, a);
        cr.fill().unwrap();
    }
}
