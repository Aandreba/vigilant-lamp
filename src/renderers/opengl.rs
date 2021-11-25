use core::panic;
use std::{collections::HashMap, fmt::format, io::Read, ops::Deref, rc::Rc, str::FromStr, time::Duration};
use gl33::{GL_ARRAY_BUFFER, GL_COLOR_BUFFER_BIT, GL_COMPILE_STATUS, GL_ELEMENT_ARRAY_BUFFER, GL_FILL, GL_FLOAT, GL_FRAGMENT_SHADER, GL_FRONT_AND_BACK, GL_LINE, GL_LINK_STATUS, GL_STATIC_DRAW, GL_TRIANGLES, GL_UNSIGNED_INT, GL_VALIDATE_STATUS, GL_VERTEX_SHADER, GLenum, global_loader::{glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor, glCompileShader, glCreateProgram, glCreateShader, glDisableVertexAttribArray, glDrawElements, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glGetUniformLocation, glLinkProgram, glPolygonMode, glShaderSource, glUniform1f, glUniform1fv, glUniform1i, glUniform1iv, glUniform1ui, glUniform1uiv, glUniform4iv, glUniformMatrix2fv, glUniformMatrix3fv, glUniformMatrix4fv, glUseProgram, glValidateProgram, glVertexAttribPointer, load_global_gl}};
use glutin::{Api, ContextBuilder, GlRequest, PossiblyCurrent, WindowedContext, dpi::LogicalSize, event::{ElementState, Event, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use crate::{engine::{camera::Camera, clock::Clock, input::keyboard::{KeyboardKey, KeyboardListener}, objectg::ObjectG, scene::{Scene}}, graph::{mesh::Mesh, renderer::{self, Renderer}, shaders::{program::{self, Program, Uniform}, shader::{FragmentShader, VertexShader}}, window::{Window}}, math::matrix::{Matrix2, Matrix3, Matrix4}};

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
    type KeyboardListenerType = KeyboardListenerGL; 

    fn run (self, mut scene: Scene<Self>) {
        scene.program.validate();

        let mut clock = Clock::new();
        let mut keyboard_listener = KeyboardListenerGL { pressed: [false; 161] };

        match scene.script.start {
            Some(x) => x(&mut scene),
            None => ()
        }

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == scene.window.context.window().id() => *control_flow = ControlFlow::Exit,

                Event::WindowEvent { event: WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ }, window_id } => {
                    let keycode = input.virtual_keycode;
                    let key = match keycode {
                        Some(x) => KEYBOARD_MAPPING[x as usize],
                        None => KeyboardKey::UNKNOWN
                    };

                    keyboard_listener.pressed[key as usize] = input.state == ElementState::Pressed
                },

                Event::RedrawRequested(_) => {
                    scene.window.clear();
                    scene.program.bind();
                    
                    let delta = clock.delta();
                    match scene.script.update {
                        Some(x) => x(&mut scene, &keyboard_listener, &delta),
                        None => ()
                    }

                    scene.program.set_float_mat4_by_name("camera", scene.camera_matrix());
                    for elem in scene.objects.iter() {
                        scene.program.set_float_mat4_by_name("world_matrix", elem.transform.matrix());
                        unsafe { OpenGL::draw_mesh_static(&elem.mesh) }
                    }

                    scene.program.unbind();
                    scene.window.update()
                },

                Event::MainEventsCleared => {
                    scene.window.context.window().request_redraw();
                }
                _ => ()
            }
        })
    }

    fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Self::WindowType {
        let window = WindowBuilder::new().with_title(title).with_inner_size(LogicalSize::new(width, height));
        let context : WindowedContext<PossiblyCurrent>;
        unsafe {
            context = ContextBuilder::new()
                    .with_gl(GlRequest::Specific(Api::OpenGl, (3,3)))
                    .with_vsync(vsync)
                    .build_windowed(window, &self.event_loop)
                    .unwrap().make_current().unwrap();
        }

        unsafe {
            load_global_gl(&|ptr| {
                let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
                let r_str = c_str.to_str().unwrap();
                context.get_proc_address(r_str) as _
            });
            
            glClearColor(0., 0., 0., 1.);
            WinitWindow { title: String::from_str(title).unwrap(), context }
        }
    }

    fn create_vertex_shader<R: Read> (&self, code: R) -> VertexGL {
        let id: Result<u32, String>;

        unsafe {
            id = self.create_shader(code, GL_VERTEX_SHADER);
        }
        
        match id {
            Ok(x) => return VertexGL(x),
            Err(x) => panic!("Vertex shader: {}", x)
        }
    }

    fn create_fragment_shader<R: Read> (&self, code: R) -> FragmentGL {
        let id: Result<u32, String>;
        unsafe {
            id = self.create_shader(code, GL_FRAGMENT_SHADER);
        }

        match id {
            Ok(x) => return FragmentGL(x),
            Err(x) => panic!("Fragment shader: {}", x)
        }
    }

    fn create_program (&self, vertex: VertexGL, fragment: FragmentGL, uniforms: &[&str]) -> ProgramGL {
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
                let mut log_string : Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0;

                glGetProgramInfoLog(id, 1024, &mut log_len, log_string.as_mut_ptr().cast());
                log_string.set_len(log_len as usize);
                panic!("{}", String::from_utf8(log_string).unwrap());
            }
        }

        // TODO UNIFORMS
        let uniform_cast : Vec<UniformGL> = uniforms.iter()
            .map(|x| UniformGL { name: String::from_str(x).unwrap(), id: unsafe { glGetUniformLocation(id, format!("{}{}", x, "\0").as_ptr() as *const _) } })
            .collect();

        ProgramGL { id, vertex, fragment, uniforms: uniform_cast }
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

    fn draw_mesh (&self, mesh: &MeshGL) {
        unsafe { OpenGL::draw_mesh_static(mesh) }
    }

    fn set_wireframe(&mut self, value: bool) {
        unsafe {
            glPolygonMode(GL_FRONT_AND_BACK, if value { GL_LINE } else { GL_FILL })
        }
    }
}

impl OpenGL {
    pub unsafe fn draw_mesh_static (mesh: &MeshGL) {
        glBindVertexArray(mesh.id);
        glEnableVertexAttribArray(0);

        glDrawElements(GL_TRIANGLES, 3 * mesh.index_count as i32, GL_UNSIGNED_INT, 0 as *const _);
        
        glDisableVertexAttribArray(0);
        glBindVertexArray(0);
    }

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
            let mut log_string : Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            
            glGetShaderInfoLog(id, 1024, &mut log_len, log_string.as_mut_ptr());
            log_string.set_len(log_len as usize);

            return Err(String::from_utf8(log_string).unwrap());
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
    uniforms: Vec<UniformGL>,
    vertex: VertexGL,
    fragment: FragmentGL
}

impl Program for ProgramGL {
    type Vertex = VertexGL;
    type Fragment = FragmentGL;
    type Uniform = UniformGL;

    fn get_vertex (&self) -> &Self::Vertex {
        &self.vertex
    }

    fn get_fragment (&self) -> &Self::Fragment {
        &self.fragment
    }

    fn validate(&self) {
        let mut success = 0;

        unsafe {
            glValidateProgram(self.id);
            glGetProgramiv(self.id, GL_VALIDATE_STATUS, &mut success);

            if success == 0 {
                let mut log_string : Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0;

                glGetProgramInfoLog(self.id, 1024, &mut log_len, log_string.as_mut_ptr().cast());
                log_string.set_len(log_len as usize);
                panic!("{}", String::from_utf8(log_string).unwrap());
            }
        }
    }

    fn get_uniforms(&self) -> &[Self::Uniform] {
        self.uniforms.as_ref()
    }

    fn set_bool (&self, key: &Self::Uniform, value: bool) {
        self.set_int(key, if value { 1 } else { 0 })
    }

    fn set_bools (&self, key: &Self::Uniform, value: &[bool]) {
        let map : Vec<i32> = value.iter().map(|x| if *x { 1 } else { 0 }).collect();
        self.set_ints(key, map.as_ref())
    }

    fn set_int(&self, key: &Self::Uniform, value: i32) {
        unsafe {
            glUniform1i(key.id, value) 
        }
    }

    fn set_ints (&self, key: &Self::Uniform, value: &[i32]) {
        unsafe {
            glUniform1iv(key.id, value.len() as i32, value.as_ptr())
        }
    }

    fn set_uint (&self, key: &Self::Uniform, value: u32) {
        unsafe {
            glUniform1ui(key.id, value)
        }
    }

    fn set_uints (&self, key: &Self::Uniform, value: &[u32]) {
        unsafe {
            glUniform1uiv(key.id, value.len() as i32, value.as_ptr())
        }
    }

    fn set_float(&self, key: &Self::Uniform, value: f32) {
        unsafe {
            glUniform1f(key.id, value)
        }
    }

    fn set_floats (&self, key: &Self::Uniform, value: &[f32]) {
        unsafe {
            glUniform1fv(key.id, value.len() as i32, value.as_ptr())
        }   
    }

    fn set_float_mat2(&self, key: &Self::Uniform, value: Matrix2<f32>) {
        unsafe {
            glUniformMatrix2fv(key.id, 1, 1, value.flat().as_ptr())
        }
    }

    fn set_float_mat3(&self, key: &Self::Uniform, value: Matrix3<f32>) {
        unsafe {
            glUniformMatrix3fv(key.id, 1, 1, value.flat().as_ptr())
        }
    }

    fn set_float_mat4(&self, key: &Self::Uniform, value: Matrix4<f32>) {
        unsafe {
            glUniformMatrix4fv(key.id, 1, 1, value.flat().as_ptr())
        }
    }

    fn set_double(&self, key: &Self::Uniform, value: f64) {
        panic!("Unsuported operation")
    }

    fn set_doubles (&self, key: &Self::Uniform, value: &[f64]) {
        panic!("Unsuported operation")  
    }

    fn set_double_mat2(&self, key: &Self::Uniform, value: Matrix2<f64>) {
        panic!("Unsuported operation")
    }

    fn set_double_mat3(&self, key: &Self::Uniform, value: Matrix3<f64>) {
        panic!("Unsuported operation")
    }

    fn set_double_mat4(&self, key: &Self::Uniform, value: Matrix4<f64>) {
        panic!("Unsuported operation")
    }

    fn bind(&self) {
        glUseProgram(self.id)
    }

    fn unbind(&self) {
        glUseProgram(0)
    }
}

// UNIFORMS
pub struct UniformGL {
    name: String,
    id: i32
}

impl Uniform for UniformGL  {
    fn get_name (&self) -> &str {
        self.name.as_str()
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
    pub context: WindowedContext<PossiblyCurrent>
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
        self.context.swap_buffers().expect("Unexpected error swaping buffers")
    }
}

// lISTENERS
const KEYBOARD_MAPPING : [KeyboardKey; 161] = [
    KeyboardKey::ONE,
    KeyboardKey::TWO,
    KeyboardKey::THREE,
    KeyboardKey::FOUR,
    KeyboardKey::FIVE,
    KeyboardKey::SIX,
    KeyboardKey::SEVEN,
    KeyboardKey::EIGHT,
    KeyboardKey::NINE,
    KeyboardKey::ZERO,

    KeyboardKey::A,
    KeyboardKey::B,
    KeyboardKey::C,
    KeyboardKey::D,
    KeyboardKey::E,
    KeyboardKey::F,
    KeyboardKey::G,
    KeyboardKey::H,
    KeyboardKey::I,
    KeyboardKey::J,
    KeyboardKey::K,
    KeyboardKey::L,
    KeyboardKey::M,
    KeyboardKey::N,
    KeyboardKey::O,
    KeyboardKey::P,
    KeyboardKey::Q,
    KeyboardKey::R,
    KeyboardKey::S,
    KeyboardKey::T,
    KeyboardKey::U,
    KeyboardKey::V,
    KeyboardKey::W,
    KeyboardKey::X,
    KeyboardKey::Y,
    KeyboardKey::Z,

    KeyboardKey::ESCAPE,

    KeyboardKey::F1,
    KeyboardKey::F2,
    KeyboardKey::F3,
    KeyboardKey::F4,
    KeyboardKey::F5,
    KeyboardKey::F6,
    KeyboardKey::F7,
    KeyboardKey::F8,
    KeyboardKey::F9,
    KeyboardKey::F10,
    KeyboardKey::F12,
    KeyboardKey::F12,
    KeyboardKey::F13,
    KeyboardKey::F14,
    KeyboardKey::F15,
    KeyboardKey::F16,
    KeyboardKey::F17,
    KeyboardKey::F18,
    KeyboardKey::F19,
    KeyboardKey::F20,
    KeyboardKey::F21,
    KeyboardKey::F22,
    KeyboardKey::F23,
    KeyboardKey::F24,

    KeyboardKey::PRINT_SCREEN,
    KeyboardKey::SCROLL_LOCK,
    KeyboardKey::PAUSE,

    KeyboardKey::INSERT,
    KeyboardKey::HOME,
    KeyboardKey::DELETE,
    KeyboardKey::END,
    KeyboardKey::PAGE_DOWN,
    KeyboardKey::PAGE_UP,

    KeyboardKey::LEFT,
    KeyboardKey::UP,
    KeyboardKey::RIGHT,
    KeyboardKey::DOWN,

    KeyboardKey::BACKSPACE,
    KeyboardKey::ENTER,
    KeyboardKey::SPACE,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::NUM_LOCK,
    KeyboardKey::KP0,
    KeyboardKey::KP1,
    KeyboardKey::KP2,
    KeyboardKey::KP3,
    KeyboardKey::KP4,
    KeyboardKey::KP5,
    KeyboardKey::KP6,
    KeyboardKey::KP7,
    KeyboardKey::KP8,
    KeyboardKey::KP9,

    KeyboardKey::KP_ADD,
    KeyboardKey::KP_DIVIDE,
    KeyboardKey::KP_DECIMAL,
    KeyboardKey::KP_DECIMAL,
    KeyboardKey::KP_ENTER,
    KeyboardKey::KP_EQUAL,
    KeyboardKey::KP_MULTIPLY,
    KeyboardKey::KP_SUBTRACT,

    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::APOSTROPHE,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    
    KeyboardKey::BACKSLASH,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::COMMA,
    KeyboardKey::UNKNOWN,
    KeyboardKey::EQUAL,
    KeyboardKey::GRAVE_ACCENT,
    KeyboardKey::UNKNOWN,
    KeyboardKey::LEFT_ALT,
    KeyboardKey::LEFT_BRACKET,
    KeyboardKey::LEFT_CONTROL,
    KeyboardKey::LEFT_SHIFT,
    KeyboardKey::UNKNOWN,
    
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::MINUS,
    KeyboardKey::UNKNOWN,

    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::PERIOD,
    KeyboardKey::UNKNOWN, // PLAY-PAUSE
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::RIGHT_ALT,
    KeyboardKey::RIGHT_BRACKET,
    KeyboardKey::RIGHT_CONTROL,
    KeyboardKey::RIGHT_SHIFT,
    KeyboardKey::UNKNOWN,

    KeyboardKey::SEMICOLON,
    KeyboardKey::SLASH,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::TAB,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN
];

pub struct KeyboardListenerGL {
    pressed: [bool; 161]
}

impl KeyboardListener for KeyboardListenerGL {
    fn is_pressed (&self, key: crate::engine::input::keyboard::KeyboardKey) -> bool {
        self.pressed[key as usize]
    }
}