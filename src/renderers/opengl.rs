use std::{str::FromStr};
use gl33::{GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER, GL_FILL, GL_FLOAT, GL_FRAGMENT_SHADER, GL_FRONT_AND_BACK, GL_LINE, GL_LINK_STATUS, GL_STATIC_DRAW, GL_TRIANGLES, GL_UNSIGNED_INT, GL_VALIDATE_STATUS, GL_VERTEX_SHADER, GLenum, global_loader::{glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor, glCompileShader, glCreateProgram, glCreateShader, glDisableVertexAttribArray, glDrawElements, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glGetUniformLocation, glLinkProgram, glPolygonMode, glShaderSource, glUniform1f, glUniform1fv, glUniform1i, glUniform1iv, glUniform1ui, glUniform1uiv, glUniform4iv, glUniformMatrix2fv, glUniformMatrix3fv, glUniformMatrix4fv, glUseProgram, glValidateProgram, glVertexAttribPointer, load_global_gl, glGenTextures, glBindTexture, glPixelStorei, glTexParameteri, glTexImage1D, glUniform2f, glUniform2fv, glUniform3fv, glUniform4fv, glUniform3f, glUniform4f}, GL_COMPILE_STATUS, GL_TEXTURE_2D, GL_UNPACK_ALIGNMENT, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_MAG_FILTER, GL_LINEAR, GL_NEAREST, GL_RGBA, GL_UNSIGNED_BYTE};
use glutin::{Api, ContextBuilder, GlRequest, PossiblyCurrent, WindowedContext, dpi::LogicalSize, event::{ElementState, Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use crate::{engine::{input::{KeyboardKey, KeyboardListener, MouseListener}, Scene}, graph::{Mesh, Renderer, shaders::{Program, Uniform, FragmentShader, VertexShader}, Window}, ResultFlatMap, Texture, shaders::UniformValue, vector::{EucVecf2, EucVecd2, EucVecd3, EucVecd4, EucVecf3, EucVecf4}, matrix::{Matf2, Matf3, Matf4, Matd2, Matd3, Matd4}};

// RENDERER
#[derive(Debug)]
pub struct OpenGL {
    pub event_loop: EventLoop<()>
}

impl OpenGL {
    pub fn new () -> OpenGL {
        OpenGL { event_loop: EventLoop::new() }
    }
}

impl Renderer for OpenGL {
    type ErrorType = String;
    type WindowType = WinitWindow;
    type ProgramType = ProgramGL;
    type MeshType = MeshGL;
    type TextureType = TextureGL;

    type KeyboardListenerType = KeyboardListenerGL; 
    type MouseListenerType = MouseListenerGL;

    fn run (mut self, mut scene: Scene<OpenGL>) -> Result<(), Self::ErrorType> {
        let scene_init = scene.init();

        match scene_init {
            Err(x) => Err(x),
            Ok((mut clock, mut keyboard, mut mouse)) => {
                self.bind_program(&scene.program);
                self.event_loop.run(move |event, _, control_flow| {
                    *control_flow = ControlFlow::Poll;
                    
                    match event {
                        // CLOSE EVENT
                        Event::WindowEvent {
                            event: WindowEvent::CloseRequested,
                            window_id,
                        } if window_id == scene.window.context.window().id() => *control_flow = ControlFlow::Exit,
        
                        // KEYBOARD EVENT
                        Event::WindowEvent { event: WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ }, window_id } => {
                            let keycode = input.virtual_keycode;
                            let key = match keycode {
                                Some(x) => KEYBOARD_MAPPING[x as usize],
                                None => KeyboardKey::UNKNOWN
                            };
        
                            keyboard.pressed[key as usize] = input.state == ElementState::Pressed
                        },
        
                        // MOUSE EVENT
                        Event::WindowEvent { event: WindowEvent::CursorMoved { device_id: _, position, modifiers: _ }, window_id } => {
                            let size = scene.window.get_size();
                            let x = 2. * position.x / (size.0 as f64) - 1.;
                            let y = 2. * position.y / (size.1 as f64) - 1.;
                            
                            // TODO FIX
                            mouse.position = EucVecf2::new(x as f32, y as f32)
                        }
        
                        // REDRAW EVENT (UPDATE)
                        Event::RedrawRequested(_) => {
                            scene.window.clear();
                            
                            let delta = clock.delta();
                            match scene.script.update {
                                Some(x) => x(&mut scene, &keyboard, &mouse, &delta),
                                None => ()
                            }
                            
                            scene.camera_matrix().set_to_program_by_name(&mut scene.program, "camera");
                            for elem in scene.objects.iter() {
                                elem.transform.matrix().set_to_program_by_name(&mut scene.program, "world_matrix");
                                elem.material.set_to_program_by_name(&mut scene.program, "material");
                                unsafe { OpenGL::draw_mesh_static(&elem.mesh) }
                            }

                            scene.window.update();
                        },
        
                        // EXIT EVENT
                        Event::MainEventsCleared => {
                            scene.window.context.window().request_redraw();
                        }
                        _ => ()
                    }
                });
            }
        }
    }

    fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Result<Self::WindowType, String> {
        let window = WindowBuilder::new().with_title(title).with_inner_size(LogicalSize::new(width, height));
        let context : WindowedContext<PossiblyCurrent>;

        unsafe {
            let builder = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3,3)))
            .with_vsync(vsync)
            .build_windowed(window, &self.event_loop)
            .flat_map(|x| x.make_current().map_err(|z| z.1));

            match builder {
                Ok(x) => context = x,
                Err(x) => return Err(x.to_string())
            }
        }

        unsafe {
            load_global_gl(&|ptr| {
                let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
                let r_str = c_str.to_str().unwrap();
                context.get_proc_address(r_str) as _
            });
            
            glClearColor(0., 0., 0., 1.);
            Ok(WinitWindow { title: String::from_str(title).unwrap(), context })
        }
    }

    fn create_vertex_shader (&self, code: &str) -> Result<VertexGL, String> {
        let id: Result<u32, String>;

        unsafe {
            id = self.create_shader(code, GL_VERTEX_SHADER);
        }
        
        match id {
            Ok(x) => Ok(VertexGL(x)),
            Err(x) => Err(x)
        }
    }

    fn create_fragment_shader (&self, code: &str) -> Result<FragmentGL, String> {
        let id: Result<u32, String>;
        unsafe {
            id = self.create_shader(code, GL_FRAGMENT_SHADER);
        }

        match id {
            Ok(x) => Ok(FragmentGL(x)),
            Err(x) => Err(x)
        }
    }

    fn create_program (&self, vertex: VertexGL, fragment: FragmentGL, uniforms: &[&str]) -> Result<ProgramGL, String> {
        let id  = glCreateProgram();
        if id == 0 {
            return Err("Error creating program".to_string())
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
                return Err(String::from_utf8(log_string).unwrap())
            }
        }

        // TODO UNIFORMS
        let uniform_cast : Vec<UniformGL> = uniforms.iter()
            .map(|x| UniformGL { name: String::from_str(x).unwrap(), id: unsafe { glGetUniformLocation(id, format!("{}{}", x, "\0").as_ptr() as *const _) } })
            .collect();

        Ok(ProgramGL { id, vertex, fragment, uniforms: uniform_cast })
    }

    fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Result<MeshGL, String> {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao); }

        if vao == 0 {
           return Err("Error creating mesh".to_string())
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

        Ok(MeshGL { id: vao, vertices: vbo, indices: idx, vertex_count: vertices.len(), index_count: indices.len() })
    }

    fn draw_mesh (&self, mesh: &MeshGL) {
        unsafe { OpenGL::draw_mesh_static(mesh) }
    }

    fn create_texture (&self, size: (u32, u32), bytes: Vec<u8>) -> Result<Self::TextureType, Self::ErrorType> {
        let mut id = 0;
        unsafe { 
            glGenTextures(1, &mut id); 
            glBindTexture(GL_TEXTURE_2D, id);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);
            glTexImage1D(GL_TEXTURE_2D, 0, GL_RGBA.0 as i32, size.0 as i32, 0, GL_RGBA, GL_UNSIGNED_BYTE, bytes.as_ptr().cast())
        }

        todo!()
    }

    fn set_wireframe(&mut self, value: bool) {
        unsafe {
            glPolygonMode(GL_FRONT_AND_BACK, if value { GL_LINE } else { GL_FILL })
        }
    }

    fn bind_program (&mut self, program: &ProgramGL) {
        glUseProgram(program.id)
    }

    fn unbind_program (&mut self, program: &ProgramGL) {
        glUseProgram(0)
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

    unsafe fn create_shader (&self, mut code: &str, typ: GLenum) -> Result<u32, String> {
        let id = glCreateShader(typ);
        if id == 0 {
            return Err(String::from_str("Error creating vertex").unwrap())
        }

        glShaderSource(id, 1, &code.as_bytes().as_ptr().cast(), &code.len().try_into().unwrap());
        glCompileShader(id);

        let mut success = 0;
        glGetShaderiv(id, GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut log_string : Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            
            glGetShaderInfoLog(id, 1024, &mut log_len, log_string.as_mut_ptr());
            log_string.set_len(log_len as usize);

            return Err(String::from_utf8(log_string).unwrap())
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
    type Error = String;
    type Vertex = VertexGL;
    type Fragment = FragmentGL;
    type Uniform = UniformGL;

    fn get_vertex (&self) -> &Self::Vertex {
        &self.vertex
    }

    fn get_fragment (&self) -> &Self::Fragment {
        &self.fragment
    }

    fn validate(&self) -> Result<(),String> {
        let mut success = 0;

        unsafe {
            glValidateProgram(self.id);
            glGetProgramiv(self.id, GL_VALIDATE_STATUS, &mut success);

            if success == 0 {
                let mut log_string : Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0;

                glGetProgramInfoLog(self.id, 1024, &mut log_len, log_string.as_mut_ptr().cast());
                log_string.set_len(log_len as usize);
                return Err(String::from_utf8(log_string).unwrap())
            }
        }

        Ok(())
    }

    fn get_uniforms(&self) -> &[Self::Uniform] {
        self.uniforms.as_ref()
    }

    fn set_bool (&mut self, key: &Self::Uniform, value: bool) {
        self.set_int(key, if value { 1 } else { 0 })
    }

    fn set_bools (&mut self, key: &Self::Uniform, value: &[bool]) {
        let map : Vec<i32> = value.iter().map(|x| if *x { 1 } else { 0 }).collect();
        self.set_ints(key, map.as_ref())
    }

    fn set_int(&mut self, key: &Self::Uniform, value: i32) {
        unsafe {
            glUniform1i(key.id, value) 
        }
    }

    fn set_ints (&mut self, key: &Self::Uniform, value: &[i32]) {
        unsafe {
            glUniform1iv(key.id, value.len() as i32, value.as_ptr())
        }
    }

    fn set_uint (&mut self, key: &Self::Uniform, value: u32) {
        unsafe {
            glUniform1ui(key.id, value)
        }
    }

    fn set_uints (&mut self, key: &Self::Uniform, value: &[u32]) {
        unsafe {
            glUniform1uiv(key.id, value.len() as i32, value.as_ptr())
        }
    }

    fn set_float(&mut self, key: &Self::Uniform, value: f32) {
        unsafe {
            glUniform1f(key.id, value)
        }
    }

    fn set_floats (&mut self, key: &Self::Uniform, value: &[f32]) {
        unsafe {
            glUniform1fv(key.id, value.len() as i32, value.as_ptr())
        }   
    }

    fn set_float_vec2 (&mut self, key: &Self::Uniform, value: &EucVecf2) {
        unsafe {
            glUniform2f(key.id, value.x, value.y)
        }
    }

    fn set_float_vec3 (&mut self, key: &Self::Uniform, value: &EucVecf3) {
        unsafe {
            glUniform3f(key.id, value.x, value.y, value.z)
        }
    }

    fn set_float_vec4 (&mut self, key: &Self::Uniform, value: &EucVecf4) {
        unsafe {
            glUniform4f(key.id, value.x, value.y, value.z, value.w)
        }
    }

    fn set_float_mat2(&mut self, key: &Self::Uniform, value: &Matf2) {
        unsafe {
            glUniformMatrix2fv(key.id, 1, 1, value.flat().as_ptr())
        }
    }

    fn set_float_mat3(&mut self, key: &Self::Uniform, value: &Matf3) {
        unsafe {
            glUniformMatrix3fv(key.id, 1, 1, value.flat().as_ptr())
        }
    }

    fn set_float_mat4(&mut self, key: &Self::Uniform, value: &Matf4) {
        unsafe {
            glUniformMatrix4fv(key.id, 1, 1, value.flat().as_ptr())
        }
    }

    fn set_double(&mut self, key: &Self::Uniform, value: f64) {
        unimplemented!()
    }

    fn set_doubles (&mut self, key: &Self::Uniform, value: &[f64]) {
        unimplemented!() 
    }

    fn set_double_vec2 (&mut self, key: &Self::Uniform, value: &EucVecd2) {
        unimplemented!()
    }

    fn set_double_vec3 (&mut self, key: &Self::Uniform, value: &EucVecd3) {
        unimplemented!()
    }

    fn set_double_vec4 (&mut self, key: &Self::Uniform, value: &EucVecd4) {
        unimplemented!()
    }

    fn set_double_mat2(&mut self, key: &Self::Uniform, value: &Matd2) {
        unimplemented!()
    }

    fn set_double_mat3(&mut self, key: &Self::Uniform, value: &Matd3) {
        unimplemented!()
    }

    fn set_double_mat4(&mut self, key: &Self::Uniform, value: &Matd4) {
        unimplemented!()
    }
}

// UNIFORMS
#[derive(Debug, Clone)]
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
#[derive(Debug)]
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

impl Window for WinitWindow {
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

    fn update (&mut self) {
        self.context.swap_buffers().expect("Unexpected error swaping buffers")
    }

    fn clear (&mut self) {
        unsafe {
            glClear(gl33::GL_COLOR_BUFFER_BIT)
        }
    }

    fn get_property(&self, key: &str) -> Option<Box<dyn std::any::Any>> {
        None
    }
}

// TEXTURE
#[derive(Debug, Clone)]
pub struct TextureGL(u32);
impl Texture for TextureGL {}

impl UniformValue for TextureGL {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_uint(key, self.0);
        true
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
    fn is_pressed (&self, key: KeyboardKey) -> bool {
        self.pressed[key as usize]
    }

    fn init () -> Self {
        KeyboardListenerGL { pressed: [false; 161] }
    }
}

pub struct MouseListenerGL {
    position: EucVecf2
}

impl MouseListener for MouseListenerGL {
    fn init () -> Self {
        MouseListenerGL { position: EucVecf2::default() }
    }

    fn relative_position (&self) -> EucVecf2 {
        self.position.clone()
    }
}