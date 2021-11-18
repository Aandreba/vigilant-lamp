use super::window::Window;

pub trait Renderer {
    type WindowType: Window;

    fn create_window (&self, title: &str, width: u32, height: u32) -> Self::WindowType;
}