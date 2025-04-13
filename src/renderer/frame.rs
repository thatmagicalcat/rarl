use cairo::Context;
use cairo::ImageSurface;

use pango::FontDescription;
use pangocairo::functions::{create_layout, show_layout};

pub struct Frame {
    pub(crate) context: Option<Context>,
}

impl Frame {
    pub(crate) fn new(surface: &ImageSurface) -> Self {
        Self {
            context: Some(Context::new(surface).expect("failed to create context")),
        }
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap()
    }

    pub fn draw_text(&self, text: &str, x: f64, y: f64, size: f64) {
        let cr = self.get_context();

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
}

impl Drop for Frame {
    fn drop(&mut self) {
        if self.context.is_some() {
            eprintln!("[Error] Frame is created but isn't rendered");
        }
    }
}
