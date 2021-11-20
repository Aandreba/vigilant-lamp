use crate::graph::{renderer::Renderer, window::Window};

// RENDERER
pub struct DebugRenderer;

impl Renderer for DebugRenderer {
    type WindowType = DebugWindow;
    type ProgramType = todo!();

    fn create_window (&self, title: &str, width: u32, height: u32) -> DebugWindow {
        DebugWindow::new(title, width, height)
    }

    fn create_vertex_shader<R: std::io::Read> (&self, code: R) -> crate::graph::shaders::program::Program::Vertex {
        todo!()
    }

    fn create_fragment_shader<R: std::io::Read> (&self, code: R) -> crate::graph::shaders::program::Program::Fragment {
        todo!()
    }

    fn create_program (&self, vertex: crate::graph::shaders::program::Program::Vertex, fragment: crate::graph::shaders::program::Program::Fragment) -> Self::ProgramType {
        todo!()
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
    fn get_title(&self) -> &str {
        self.title.as_str()    
    }

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