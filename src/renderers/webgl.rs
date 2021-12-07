use std::rc::Rc;
use std::str::FromStr;

use game_loop::game_loop;
use js_sys::JsString;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;
use web_sys::{WebGlProgram, WebGlShader};

use crate::ErrorType;
use crate::FlatMap;
use crate::Flattern;
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
    pub fn new (title: &str) -> Result<(WebGL, WindowWGL), JsValue> {
        let window : web_sys::Window = web_sys::window().unwrap();
        let document : web_sys::Document = window.document().unwrap();
        let config = JsValue::from_serde("{ antialias: false }").unwrap();

        let element : Result<Element, JsValue> = document.query_selector(title)
            .flattern_single(|| JsValue::from_str("Element not found"));

        let canvas : Result<HtmlCanvasElement, JsValue> = element
            .flat_map(|x| x.dyn_into::<HtmlCanvasElement>())
            .map_err(|e| e.map_to_first(|x| JsValue::from_str("Element provided isn't a canvas")));

        match canvas {
            Err(x) => Err(x),
            Ok(canvas) => {
                let context : Result<WebGl2RenderingContext, JsValue> = canvas
                    .get_context_with_context_options("webgl2", &config)
                    .flattern_single(|| JsValue::from_str("WebGL v2 not available in yout browser"))
                    .flat_map(|x| x.dyn_into::<WebGl2RenderingContext>())
                    .map_err(|e| e.map_to_first(|x| JsValue::from_str("Context provided isn't WebGL v2")));
                
                match context {
                    Err(x) => Err(x),
                    Ok(context) => {
                        let renderer = WebGL { window, context: Rc::new(context), wireframe: false };
                        let window = WindowWGL::new(&renderer.context, title, canvas);
                        
                        renderer.context.clear_color(0., 0., 0., 1.);
                        renderer.context.viewport(0, 0, window.get_width() as i32, window.get_height() as i32);
                        
                        Ok((renderer, window))
                    }
                }
            }
        }
    }

    fn create_shader (&self, typ: u32, code: &str) -> Result<WebGlShader, JsValue> {
        let shader = self.context.create_shader(typ);
        match shader {
            None => Err(JsValue::from_str("Error creating creating shader")),
            Some(shader) => {
                self.context.shader_source(&shader, code);
                self.context.compile_shader(&shader);

                let compile_status = self.context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false);
                if compile_status { 
                    return Ok(shader)
                }

                let info : Option<String> = self.context.get_shader_info_log(&shader);
                match info {
                    None => Err(JsValue::from_str("Unknown error creating shader")),
                    Some(x) => Err(JsValue::from_str(x.as_str()))
                }
            }
        }
    }

    fn create_buffer_f32 (&self, values: &[f32]) -> Result<WebGlBuffer, JsValue> {
        let buffer : Option<WebGlBuffer> = self.context.create_buffer();
        match buffer {
            None => Err(JsValue::from_str("Error creating buffer")),
            Some(buffer) => {
                self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        
                unsafe {
                    let array = js_sys::Float32Array::view(values);
                    self.context.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER,
                        &array,
                        WebGl2RenderingContext::STATIC_DRAW)
                }
        
                Ok(buffer)
            }
        }
    }

    fn create_buffer_u32 (&self, values: &[u32]) -> Result<WebGlBuffer, JsValue> {
        let buffer : Option<WebGlBuffer> = self.context.create_buffer();
        match buffer {
            None => Err(JsValue::from_str("Error creating buffer")),
            Some(buffer) => {
                self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        
                unsafe {
                    let array = js_sys::Uint32Array::view(values);
                    self.context.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER,
                        &array,
                        WebGl2RenderingContext::STATIC_DRAW)
                }
        
                Ok(buffer)
            }
        }
    }
}

impl Renderer for WebGL {
    type ErrorType = JsValue;
    type WindowType = WindowWGL;
    type ProgramType = ProgramWGL;
    type MeshType = MeshWGL;
    type KeyboardListenerType = KeyboardListenerWGL;
    type MouseListenerType = MouseListenerWGL;

    fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Result<WindowWGL, JsValue> {
        WebGL::new(title).map(|x| x.1)
    }

    fn create_program (&self, vertex: VertexWGL, fragment: FragmentWGL, uniforms: &[&str]) -> Result<ProgramWGL, JsValue> {
        let program : Option<WebGlProgram> = self.context.create_program();
        match program {
            None => Err(JsValue::from_str("Error creating program")),
            Some(program) => {
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
                    return Ok(ProgramWGL { context: Rc::clone(&self.context), program, vertex, fragment, uniforms: uniform_map })
                }
                
                let err : Option<String> = self.context
                .get_program_info_log(&program);

                match err {
                    None => Err(JsValue::from_str("Unknown error creating program object")),
                    Some(x) => Err(JsValue::from_str(x.as_str()))
                }
            }
        }
    }

    fn bind_program (&self, program: &Self::ProgramType) {
        self.context.use_program(Some(&program.program))
    }

    fn unbind_program (&self, program: &ProgramWGL) {
        self.context.use_program(None)
    }

    fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Result<MeshWGL, JsValue> {
        let vao : Option<WebGlVertexArrayObject> = self.context.create_vertex_array();
        match vao {
            None => Err(JsValue::from_str("Error creating mesh")),
            Some(vao) => {
                self.context.bind_vertex_array(Some(&vao));
                let flat_vertices : Vec<f32> = vertices.iter().flat_map(|x| *x).collect();
                let vertex = self.create_buffer_f32(flat_vertices.as_slice());
                
                match vertex {
                    Err(x) => Err(x),
                    Ok(vertex) => {
                        self.context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
                        self.context.enable_vertex_attrib_array(0);

                        let flat_indices : Vec<u32> = indices.iter().flat_map(|x| *x).collect();
                        self.create_buffer_u32(flat_indices.as_slice())
                            .map(|index| MeshWGL { id: vao, vertices: vertex, indices: index, vertex_count: vertices.len(), index_count: indices.len() })
                    }
                }
            }
        }
    }

    fn draw_mesh (&self, mesh: &MeshWGL) {
        self.context.bind_vertex_array(Some(&mesh.id));
        self.context.enable_vertex_attrib_array(0);

        self.context.draw_elements_with_f64(if self.wireframe { WebGl2RenderingContext::LINES } else { WebGl2RenderingContext::TRIANGLES }, 3 * mesh.get_index_count() as i32, WebGl2RenderingContext::UNSIGNED_INT, 0.);
        
        self.context.disable_vertex_attrib_array(0);
        self.context.bind_vertex_array(None);
    }

    fn create_vertex_shader (&self, code: &str) -> Result<VertexWGL, JsValue> {
        self.create_shader(WebGl2RenderingContext::VERTEX_SHADER, code).map(|x| VertexWGL(x))
    }

    fn create_fragment_shader (&self, code: &str) -> Result<FragmentWGL, JsValue> {
        self.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER, code).map(|x| FragmentWGL(x))
    }

    fn set_wireframe (&mut self, value: bool) {
        self.wireframe = value
    }
    
    fn run (self, mut scene: Scene<WebGL>) -> Result<(), JsValue> {
        match scene.program.validate() {
            Err(x) => Err(x),
            Ok(_) => {
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

                Ok(())
            }
        }
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
    type Error = JsValue;
    type Vertex = VertexWGL;
    type Fragment = FragmentWGL;
    type Uniform = UniformWGL;

    fn get_vertex (&self) -> &VertexWGL {
        &self.vertex
    }

    fn get_fragment (&self) -> &FragmentWGL {
        &self.fragment
    }

    fn validate (&self) -> Result<(), JsValue> {
        self.context.validate_program(&self.program);

        let success = self.context
        .get_program_parameter(&self.program, WebGl2RenderingContext::VALIDATE_STATUS)
        .as_bool()
        .unwrap_or(false);
        
        if !success {
            let err : Option<String> = self.context.get_program_info_log(&self.program);
            match err {
                None => return Err(JsValue::from_str("Unknown error creating program object")),
                Some(x) => return Err(JsValue::from_str(x.as_str()))
            }
        }

        Ok(())
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