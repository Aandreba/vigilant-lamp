use super::{renderer::Renderer, window::Window};

// RENDERER
pub struct DebugRenderer;

impl Renderer for DebugRenderer {
    type WindowType = DebugWindow;

    fn create_window (&self, title: &str, width: u32, height: u32) -> DebugWindow {
        DebugWindow::new(title, width, height)
    }
}

// WINDOW
pub struct DebugWindow {
    title: String,
    width: u32,
    height: u32
}

impl DebugWindow {
    pub fn new (title: &str, width: u32, height: u32) -> DebugWindow {
        DebugWindow { title: title.to_string(), width, height }
    }
}

impl Window for DebugWindow {
    fn get_width (&self) -> u32 {
        self.width
    }

    fn get_height (&self) -> u32 {
       self.height
    }

    fn get_size (&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn update(&self) {
        todo!()
    }
}