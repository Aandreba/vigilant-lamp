use std::rc::Rc;
use std::str::FromStr;

use game_loop::game_loop;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;
use web_sys::{WebGlProgram, WebGlShader};

use crate::engine::Clock;
use crate::engine::input::KeyboardListener;
use crate::engine::input::MouseListener;
use crate::engine::Scene;
use crate::graph::Window;
use crate::graph::{Renderer, shaders::{Program, Uniform, VertexShader, FragmentShader}, Mesh};
use crate::input::KeyboardKey;
use crate::math::array_ext::NumArray;

pub struct WebGL {
    window: web_sys::Window,
    context: Rc<WebGl2RenderingContext>,
    wireframe: bool
}

pub struct WindowWGL {
    selector: String,
    canvas: HtmlCanvasElement,
    context: Rc<WebGl2RenderingContext>
}

impl WindowWGL {
    pub fn new (context: &Rc<WebGl2RenderingContext>, selector: &str, canvas: HtmlCanvasElement) -> WindowWGL {
        let result = WindowWGL {
            context: Rc::clone(context), 
            selector: String::from_str(selector).unwrap(), 
            canvas
        };

        result
    }
}

impl WebGL {
    pub fn new (title: &str) -> (WebGL, WindowWGL) {
        let window : web_sys::Window = web_sys::window().unwrap();
        let document : web_sys::Document = window.document().unwrap();
        let config = JsValue::from_serde("{ antialias: false }").unwrap();

        let element = document.query_selector(title).expect("Unexpected error").expect("DOM Element not found");
        let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>().expect("Unexpected error");

        let context = canvas.get_context_with_context_options("webgl2", &config).unwrap().unwrap();
        let context : WebGl2RenderingContext = context.dyn_into::<WebGl2RenderingContext>().expect("Unexpected error");

        let renderer = WebGL { window, context: Rc::new(context), wireframe: false };
        let window = WindowWGL::new(&renderer.context, title, canvas);
        
        renderer.context.clear_color(0., 0., 0., 1.);
        renderer.context.viewport(0, 0, window.get_width() as i32, window.get_height() as i32);
        
        (renderer, window)
    }

    fn create_shader (&self, typ: u32, code: &str) -> Result<WebGlShader, String> {
        let shader = self.context.create_shader(typ).expect("Error creating shader");
        self.context.shader_source(&shader, code);
        self.context.compile_shader(&shader);

        let compile_status = self.context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false);
        if compile_status { 
            return Ok(shader)
        }

        Err(self.context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
    }

    fn create_buffer_f32 (&self, values: &[f32]) -> WebGlBuffer {
        let buffer = self.context.create_buffer().expect("Error creating buffer");
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        
        unsafe {
            let array = js_sys::Float32Array::view(values);
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array,
                WebGl2RenderingContext::STATIC_DRAW)
        }

        buffer
    }

    fn create_buffer_u32 (&self, values: &[u32]) -> WebGlBuffer {
        let buffer = self.context.create_buffer().expect("Error creating buffer");
        self.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        
        unsafe {
            let array = js_sys::Uint32Array::view(values);
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &array,
                WebGl2RenderingContext::STATIC_DRAW)
        }

        buffer
    }
}

impl Renderer for WebGL {
    type WindowType = WindowWGL;
    type ProgramType = ProgramWGL;
    type MeshType = MeshWGL;
    type KeyboardListenerType = KeyboardListenerWGL;
    type MouseListenerType = MouseListenerWGL;

    fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> WindowWGL {
        WebGL::new(title).1
    }

    fn create_program (&self, vertex: VertexWGL, fragment: FragmentWGL, uniforms: &[&str]) -> ProgramWGL {
        let program = self.context.create_program().expect("Error creating program");
        self.context.attach_shader(&program, &vertex.0);
        self.context.attach_shader(&program, &fragment.0);
        self.context.link_program(&program);

        let uniform_map : Vec<UniformWGL> = uniforms.iter()
            .map(|x| UniformWGL { id: self.context.get_uniform_location(&program, x), name: String::from_str(*x).unwrap() }).collect();
        
        let link_status = self.context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);

        if link_status {
            return ProgramWGL { context: Rc::clone(&self.context), program, vertex, fragment, uniforms: uniform_map }
        }
        
        let err = self.context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object"));

        panic!("{}", err)
    }

    fn bind_program (&self, program: &Self::ProgramType) {
        self.context.use_program(Some(&program.program))
    }

    fn unbind_program (&self, program: &ProgramWGL) {
        self.context.use_program(None)
    }

    fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> MeshWGL {
        let vao = self.context.create_vertex_array().expect("Error creating mesh");
        self.context.bind_vertex_array(Some(&vao));

        let flat_vertices : Vec<f32> = vertices.iter().flat_map(|x| *x).collect();
        let flat_indices : Vec<u32> = indices.iter().flat_map(|x| *x).collect();

        let vertex = self.create_buffer_f32(flat_vertices.as_slice());
        self.context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(0);
        
        let index = self.create_buffer_u32(flat_indices.as_slice());
        MeshWGL { id: vao, vertices: vertex, indices: index, vertex_count: vertices.len(), index_count: indices.len() }
    }

    fn draw_mesh (&self, mesh: &MeshWGL) {
        self.context.bind_vertex_array(Some(&mesh.id));
        self.context.enable_vertex_attrib_array(0);

        self.context.draw_elements_with_f64(if self.wireframe { WebGl2RenderingContext::LINES } else { WebGl2RenderingContext::TRIANGLES }, 3 * mesh.get_index_count() as i32, WebGl2RenderingContext::UNSIGNED_INT, 0.);
        
        self.context.disable_vertex_attrib_array(0);
        self.context.bind_vertex_array(None);
    }

    fn create_vertex_shader (&self, code: &str) -> VertexWGL {
        let shader = self.create_shader(WebGl2RenderingContext::VERTEX_SHADER, code);
        match shader {
            Ok(x) => return VertexWGL(x),
            Err(x) => panic!("{}", x)
        }
    }

    fn create_fragment_shader (&self, code: &str) -> FragmentWGL {
        let shader = self.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER, code);
        match shader {
            Ok(x) => return FragmentWGL(x),
            Err(x) => panic!("{}", x)
        }
    }

    fn set_wireframe (&mut self, value: bool) {
        self.wireframe = value
    }
    
    fn run (self, mut scene: Scene<WebGL>) {
        scene.program.validate();

        let mut clock = Clock::new();
        let keyboard_listener = KeyboardListenerWGL([false; 161]);
        let mouse_listener = MouseListenerWGL(NumArray::zero());

        match scene.script.start {
            Some(x) => x(&mut scene),
            None => ()
        }

        game_loop((self, scene, clock, keyboard_listener, mouse_listener), 240, 0.1, move |g| {
            let delta = clock.delta();
            match g.game.1.script.update {
                Some(x) => x(&mut g.game.1, &g.game.3, &g.game.4, &delta),
                None => ()
            }
        }, |g| {
            g.game.0.render(&mut g.game.1)
        });
    }
}

// WINDOW
impl Window for WindowWGL {
    fn get_title (&self) -> &str {
        self.selector.as_str()
    }

    fn get_width (&self) -> u32 {
        self.canvas.width()
    }

    fn get_height (&self) -> u32 {
        self.canvas.height()
    }

    fn get_size (&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }

    fn clear (&self) {
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT)
    } 

    fn update (&mut self) {
        self.context.viewport(0, 0, self.get_width() as i32, self.get_height() as i32)
    }
}

// UNFIFORM
#[wasm_bindgen]
pub struct UniformWGL {
    id: Option<WebGlUniformLocation>,
    name: String
}

impl Uniform for UniformWGL {
    fn get_name (&self) -> &str {
        self.name.as_str()
    }
}

// PROGRAM
#[wasm_bindgen]
pub struct ProgramWGL {
    context: Rc<WebGl2RenderingContext>,
    program: WebGlProgram,
    vertex: VertexWGL,
    fragment: FragmentWGL,
    uniforms: Vec<UniformWGL>
}

impl Program for ProgramWGL {
    type Vertex = VertexWGL;
    type Fragment = FragmentWGL;
    type Uniform = UniformWGL;

    fn get_vertex (&self) -> &VertexWGL {
        &self.vertex
    }

    fn get_fragment (&self) -> &FragmentWGL {
        &self.fragment
    }

    fn validate (&self) {
        self.context.validate_program(&self.program);

        let success = self.context
        .get_program_parameter(&self.program, WebGl2RenderingContext::VALIDATE_STATUS)
        .as_bool()
        .unwrap_or(false);
        
        if !success {
            let err = self.context
                .get_program_info_log(&self.program)
                .unwrap_or_else(|| String::from("Unknown error creating program object"));
            
            panic!("{}", err)
        }
    }

    fn get_uniforms (&self) -> &[UniformWGL] {
        self.uniforms.as_slice()
    }

    fn set_bool (&self, key: &UniformWGL, value: bool) {
        self.set_int(key, if value { 1 } else { 0 })
    }

    fn set_int (&self, key: &UniformWGL, value: i32) {
        self.context.uniform1i(key.id.as_ref(), value)
    }

    fn set_uint (&self, key: &UniformWGL, value: u32) {
        self.context.uniform1ui(key.id.as_ref(), value)
    }

    fn set_float (&self, key: &UniformWGL, value: f32) {
        self.context.uniform1f(key.id.as_ref(), value)
    }

    fn set_double (&self, key: &UniformWGL, value: f64) {
        unimplemented!()
    }

    fn set_bools (&self, key: &UniformWGL, value: &[bool]) {
        let map : Vec<i32> = value.iter().map(|x| if *x { 1 } else { 0 }).collect();
        self.set_ints(key, map.as_ref())
    }

    fn set_ints (&self, key: &UniformWGL, value: &[i32]) {
        self.context.uniform1iv_with_i32_array(key.id.as_ref(), value)
    }

    fn set_uints (&self, key: &UniformWGL, value: &[u32]) {
        self.context.uniform1uiv_with_u32_array(key.id.as_ref(), value)
    }

    fn set_floats (&self, key: &UniformWGL, value: &[f32]) {
        self.context.uniform1fv_with_f32_array(key.id.as_ref(), value)
    }

    fn set_doubles (&self, key: &UniformWGL, value: &[f64]) {
        unimplemented!()
    }

    fn set_float_mat2 (&self, key: &Self::Uniform, value: crate::math::matrix::Matrix2<f32>) {
        self.context.uniform_matrix2fv_with_f32_array(key.id.as_ref(), true, value.flat().as_ref())
    }

    fn set_float_mat3 (&self, key: &Self::Uniform, value: crate::math::matrix::Matrix3<f32>) {
        self.context.uniform_matrix3fv_with_f32_array(key.id.as_ref(), true, value.flat().as_ref())
    }

    fn set_float_mat4 (&self, key: &Self::Uniform, value: crate::math::matrix::Matrix4<f32>) {
        self.context.uniform_matrix4fv_with_f32_array(key.id.as_ref(), true, value.flat().as_ref())
    }

    fn set_double_mat2 (&self, key: &Self::Uniform, value: crate::math::matrix::Matrix2<f64>) {
        unimplemented!()
    }

    fn set_double_mat3 (&self, key: &Self::Uniform, value: crate::math::matrix::Matrix3<f64>) {
        unimplemented!()
    }

    fn set_double_mat4 (&self, key: &Self::Uniform, value: crate::math::matrix::Matrix4<f64>) {
        unimplemented!()
    }
}

// SHADER
pub struct VertexWGL (WebGlShader);

pub struct FragmentWGL (WebGlShader);

impl VertexShader for VertexWGL {}
impl FragmentShader for FragmentWGL {}

// MESH
pub struct MeshWGL {
    id: WebGlVertexArrayObject,
    vertices: WebGlBuffer,
    indices: WebGlBuffer,

    vertex_count: usize,
    index_count: usize
}

impl Mesh for MeshWGL {
    fn get_vertex_count (&self) -> usize {
        self.vertex_count
    }

    fn get_index_count (&self) -> usize {
        self.index_count
    }
}

// LISTENERS
pub struct KeyboardListenerWGL ([bool;161]);

impl KeyboardListenerWGL {
    pub fn new () -> KeyboardListenerWGL {
        KeyboardListenerWGL([false; 161])
    }
}

pub struct MouseListenerWGL (NumArray<f32,2>);

impl MouseListenerWGL {
    pub fn new () -> MouseListenerWGL {
        MouseListenerWGL(NumArray::zero())
    }
}

impl KeyboardListener for KeyboardListenerWGL {
    fn is_pressed (&self, key: KeyboardKey) -> bool {
        self.0[key as usize]
    }
}

impl MouseListener for MouseListenerWGL {
    fn relative_position (&self) -> NumArray<f32, 2> {
        self.0
    }
}