use cairo::Context;
use cairo::ImageSurface;

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
}

impl Drop for Frame {
    fn drop(&mut self) {
        if self.context.is_some() {
            eprintln!("[Error] Frame is created but isn't rendered");
        }
    }
}
