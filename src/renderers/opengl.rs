use std::{io::Read, str::FromStr};

use gl33::{GL_ARRAY_BUFFER, GL_COLOR_BUFFER_BIT, GL_COMPILE_STATUS, GL_ELEMENT_ARRAY_BUFFER, GL_FLOAT, GL_FRAGMENT_SHADER, GL_LINK_STATUS, GL_STATIC_DRAW, GL_TRIANGLES, GL_UNSIGNED_INT, GL_VALIDATE_STATUS, GL_VERTEX_SHADER, GLenum, global_loader::{glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glCompileShader, glCreateProgram, glCreateShader, glDrawElements, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glLinkProgram, glShaderSource, glUseProgram, glValidateProgram, glVertexAttribPointer}};
use glutin::{ContextBuilder, ContextWrapper, NotCurrent, PossiblyCurrent, WindowedContext, dpi::LogicalSize, event_loop::EventLoop, window::{Window, WindowBuilder}};
use crate::graph::{mesh::Mesh, renderer::Renderer, shaders::{program::Program, shader::{FragmentShader, VertexShader}}};

// RENDERER
pub struct OpenGL {
    pub event_loop: EventLoop<()>
}

impl OpenGL {
    pub fn new () -> OpenGL {
        OpenGL { event_loop: EventLoop::new() }
    }
}

impl Renderer for OpenGL {
    type WindowType = WinitWindow;
    type ProgramType = ProgramGL;
    type MeshType = MeshGL;

    fn create_window (&self, title: &str, width: u32, height: u32) -> Self::WindowType {
        let window = WindowBuilder::new().with_title(title).with_inner_size(LogicalSize::new(width, height));
        let context = ContextBuilder::new().build_windowed(window, &self.event_loop);

        match context {
            Ok(context ) => return WinitWindow { title: String::from_str(title).unwrap(), context: unsafe { context.make_current().unwrap() } },
            Err(x) => panic!("{}", x)
        }
    }

    fn create_vertex_shader<R: Read> (&self, code: R) -> VertexGL {
        let id: Result<u32, String>;

        unsafe {
            id = self.create_shader(code, GL_VERTEX_SHADER);
        }
        
        match id {
            Ok(x) => return VertexGL(x),
            Err(x) => panic!("{}", x)
        }
    }

    fn create_fragment_shader<R: Read> (&self, code: R) -> FragmentGL {
        let id: Result<u32, String>;
        unsafe {
            id = self.create_shader(code, GL_FRAGMENT_SHADER);
        }

        match id {
            Ok(x) => return FragmentGL(x),
            Err(x) => panic!("{}", x)
        }
    }

    fn create_program (&self, vertex: VertexGL, fragment: FragmentGL) -> ProgramGL {
        let id  = glCreateProgram();
        if id == 0 {
            panic!("Error creating program");
        }

        glAttachShader(id, vertex.0);
        glAttachShader(id, fragment.0);

        // LINK
        glLinkProgram(id);
        let mut success = 0;

        unsafe {
            glGetProgramiv(id, GL_LINK_STATUS, &mut success);
            if success == 0 {
                let mut log_string = String::new();
                let mut log_len = 0_i32;

                glGetShaderInfoLog(id, 1024, &mut log_len, log_string.as_mut_ptr().cast());
                panic!("{}", log_string);
            }
        }

        // VALIDATE
        unsafe {
            glValidateProgram(id);
            glGetProgramiv(id, GL_VALIDATE_STATUS, &mut success);

            if success == 0 {
                let mut log_string = String::new();
                let mut log_len = 0_i32;

                glGetShaderInfoLog(id, 1024, &mut log_len, log_string.as_mut_ptr().cast());
                panic!("{}", log_string);
            }
        }

        ProgramGL { id, vertex, fragment }
    }

    fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> MeshGL {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao); }
        if vao == 0 {
            panic!("Error creating mesh");
        }

        glBindVertexArray(vao);
        let vbo: u32;
        let idx: u32;

        unsafe { 
            vbo = self.buffer_data( GL_ARRAY_BUFFER, bytemuck::cast_slice(vertices)); 
            glVertexAttribPointer(0, 3, GL_FLOAT, 0, 0, 0 as *const _);
            glEnableVertexAttribArray(0);
            idx = self.buffer_data( GL_ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(indices));
        }

        MeshGL { id: vao, vertices: vbo, indices: idx, vertex_count: vertices.len(), index_count: indices.len() }
    }

    fn draw_mesh (&self, mesh: MeshGL) {
        unsafe {
            glDrawElements(GL_TRIANGLES, mesh.vertex_count as i32, GL_UNSIGNED_INT, 0 as *const _)
        }
    }
}

impl OpenGL {
    unsafe fn buffer_data (&self, typ: GLenum, data: &[u8]) -> u32 {
        let mut id = 0;

        glGenBuffers(1, &mut id);
        glBindBuffer(typ, id);
        glBufferData(typ, data.len().try_into().unwrap(), data.as_ptr().cast(), GL_STATIC_DRAW);
        id
    }

    unsafe fn create_shader<R: Read> (&self, mut code: R, typ: GLenum) -> Result<u32, String> {
        let id = glCreateShader(typ);
        if id == 0 {
            return Err(String::from_str("Error creating vertex").unwrap())
        }
        
        let mut code_string = String::new();
        code.read_to_string(&mut code_string);
        
        glShaderSource(id, 1, &code_string.as_bytes().as_ptr().cast(), &code_string.len().try_into().unwrap());
        glCompileShader(id);

        let mut success = 0;
        glGetShaderiv(id, GL_COMPILE_STATUS, &mut success);
        
        if success == 0 {
            let mut log_string = String::new();
            let mut log_len = 0_i32;

            glGetShaderInfoLog(id, 1024, &mut log_len, log_string.as_mut_ptr().cast());
            return Err(log_string);
        }

        return Ok(id);
    }

}

// SHADERS
pub struct VertexGL(u32);
pub struct FragmentGL(u32);

impl VertexShader for VertexGL {}
impl FragmentShader for FragmentGL {}

// SHADER PROGRAM
pub struct ProgramGL {
    id: u32,
    vertex: VertexGL,
    fragment: FragmentGL
}

impl Program for ProgramGL {
    type Vertex = VertexGL;
    type Fragment = FragmentGL;

    fn get_vertex (&self) -> &Self::Vertex {
        &self.vertex
    }

    fn get_fragment (&self) -> &Self::Fragment {
        &self.fragment
    }

    fn bind(&self) {
        glUseProgram(self.id)
    }

    fn unbind(&self) {
        glUseProgram(0)
    }
}

// MESH
pub struct MeshGL {
    id: u32,
    vertices: u32,
    indices: u32,

    vertex_count: usize,
    index_count: usize,
}

impl Mesh for MeshGL {
    fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }

    fn get_index_count(&self) -> usize {
        self.index_count
    }
}

// WINDOW
pub struct WinitWindow {
    title: String,
    context: WindowedContext<PossiblyCurrent>
}

impl crate::graph::window::Window for WinitWindow {
    fn get_title (&self) -> &str {
        self.title.as_str()
    }

    fn get_width (&self) -> u32 {
        self.context.window().inner_size().width
    }

    fn get_height (&self) -> u32 {
        self.context.window().inner_size().height
    }

    fn get_size (&self) -> (u32, u32) {
        let size = self.context.window().inner_size();
        (size.width, size.height)
    }

    fn clear(&self) {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
        }
    }

    fn update (&self) {
        
    }
}