use std::any::Any;
use std::any::TypeId;
use std::rc::Rc;
use std::str::FromStr;

use game_loop::game_loop;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;
use web_sys::{WebGlProgram, WebGlShader};

use crate::Texture;
use crate::matrix::Matd2;
use crate::matrix::Matd3;
use crate::matrix::Matd4;
use crate::matrix::Matf2;
use crate::matrix::Matf3;
use crate::matrix::Matf4;
use crate::shaders::UniformValue;
use crate::vector::EucVecd2;
use crate::vector::EucVecd3;
use crate::vector::EucVecd4;
use crate::vector::EucVecf2;
use crate::vector::EucVecf3;
use crate::vector::EucVecf4;
use crate::{Flattern, OptionFlatMap, ResultFlatMap};
use crate::engine::Clock;
use crate::engine::input::KeyboardListener;
use crate::engine::input::MouseListener;
use crate::engine::Scene;
use crate::graph::Window;
use crate::graph::{Renderer, shaders::{Program, Uniform, VertexShader, FragmentShader}, Mesh};
use crate::input::KeyboardKey;

#[derive(Debug)]
struct SharedData {
    window: web_sys::Window,
    context: WebGl2RenderingContext
}

#[derive(Debug, Clone)]
pub struct WebGL {
    data: Rc<SharedData>,
    wireframe: bool
}

#[derive(Debug)]
pub struct WindowWGL {
    selector: String,
    canvas: HtmlCanvasElement,
    data: Rc<SharedData>
}

impl WindowWGL {
    fn new (data: Rc<SharedData>, selector: &str, canvas: HtmlCanvasElement) -> WindowWGL {
        WindowWGL {
            selector: selector.to_string(), 
            data,
            canvas
        }
    }
}

impl WebGL {
    pub fn new (title: &str) -> Result<(WebGL, WindowWGL), JsValue> {
        let window : web_sys::Window = web_sys::window().expect("Window not found");
        let document : web_sys::Document = window.document().expect("Document not found");
        let config = JsValue::from_serde("{ antialias: false }").expect("Error parsing configuration");

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
                        let data = SharedData { window, context };
                        let renderer = WebGL { data: Rc::new(data), wireframe: false };
                        let window = WindowWGL::new(renderer.data.clone(), title, canvas);
                        
                        renderer.data.context.clear_color(0., 0., 0., 1.);
                        renderer.data.context.viewport(0, 0, window.get_width() as i32, window.get_height() as i32);
                        
                        Ok((renderer, window))
                    }
                }
            }
        }
    }

    fn create_shader (&self, typ: u32, code: &str) -> Result<WebGlShader, JsValue> {
        let shader = self.data.context.create_shader(typ);
        match shader {
            None => Err(JsValue::from_str("Error creating creating shader")),
            Some(shader) => {
                self.data.context.shader_source(&shader, code);
                self.data.context.compile_shader(&shader);

                let compile_status = self.data.context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false);
                if compile_status { 
                    return Ok(shader)
                }

                let info : Option<String> = self.data.context.get_shader_info_log(&shader);
                match info {
                    None => Err(JsValue::from_str("Unknown error creating shader")),
                    Some(x) => Err(JsValue::from_str(x.as_str()))
                }
            }
        }
    }

    fn create_buffer_f32 (&self, values: &[f32]) -> Result<WebGlBuffer, JsValue> {
        let buffer : Option<WebGlBuffer> = self.data.context.create_buffer();
        match buffer {
            None => Err(JsValue::from_str("Error creating buffer")),
            Some(buffer) => {
                self.data.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        
                unsafe {
                    let array = js_sys::Float32Array::view(values);
                    self.data.context.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER,
                        &array,
                        WebGl2RenderingContext::STATIC_DRAW)
                }
        
                Ok(buffer)
            }
        }
    }

    fn create_buffer_u32 (&self, values: &[u32]) -> Result<WebGlBuffer, JsValue> {
        let buffer : Option<WebGlBuffer> = self.data.context.create_buffer();
        match buffer {
            None => Err(JsValue::from_str("Error creating buffer")),
            Some(buffer) => {
                self.data.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        
                unsafe {
                    let array = js_sys::Uint32Array::view(values);
                    self.data.context.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                        &array,
                        WebGl2RenderingContext::STATIC_DRAW)
                }
        
                Ok(buffer)
            }
        }
    }

    fn render (&mut self, scene: &mut Scene<WebGL>) {
        scene.window.update();
        scene.window.clear();

        self.bind_program(&scene.program);
        scene.camera_matrix().set_to_program(&mut scene.program, "camera");
        
        for elem in scene.objects.iter() {
            elem.transform.matrix().set_to_program(&mut scene.program, "world_matrix");
            elem.material.set_to_program(&mut scene.program, "material");
            self.draw_mesh(&elem.mesh)
        }

        self.unbind_program(&scene.program);
    }
}

impl Renderer for WebGL {
    type ErrorType = JsValue;
    type WindowType = WindowWGL;
    type ProgramType = ProgramWGL;
    type MeshType = MeshWGL;
    type TextureType = WebGlTexture;
    type KeyboardListenerType = KeyboardListenerWGL;
    type MouseListenerType = MouseListenerWGL;

    fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Result<WindowWGL, JsValue> {
        WebGL::new(title).map(|x| x.1)
    }

    fn create_program (&self, vertex: VertexWGL, fragment: FragmentWGL, uniforms: &[&str]) -> Result<ProgramWGL, JsValue> {
        let program : Option<WebGlProgram> = self.data.context.create_program();
        match program {
            None => Err(JsValue::from_str("Error creating program")),
            Some(program) => {
                self.data.context.attach_shader(&program, &vertex.0);
                self.data.context.attach_shader(&program, &fragment.0);
                self.data.context.link_program(&program);

                let uniform_map : Vec<UniformWGL> = uniforms.iter()
                    .map(|x| UniformWGL { id: self.data.context.get_uniform_location(&program, x), name: String::from_str(*x).unwrap() }).collect();
                
                let link_status = self.data.context
                .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
                .as_bool()
                .unwrap_or(false);

                if link_status {
                    return Ok(ProgramWGL { data: self.data.clone(), program, vertex, fragment, uniforms: uniform_map })
                }
                
                let err : Option<String> = self.data.context
                .get_program_info_log(&program);

                match err {
                    None => Err(JsValue::from_str("Unknown error creating program object")),
                    Some(x) => Err(JsValue::from_str(x.as_str()))
                }
            }
        }
    }

    fn bind_program (&mut self, program: &Self::ProgramType) {
        self.data.context.use_program(Some(&program.program))
    }

    fn unbind_program (&mut self, program: &ProgramWGL) {
        self.data.context.use_program(None)
    }

    fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Result<MeshWGL, JsValue> {
        let vao : Option<WebGlVertexArrayObject> = self.data.context.create_vertex_array();
        match vao {
            None => Err(JsValue::from_str("Error creating mesh")),
            Some(vao) => {
                self.data.context.bind_vertex_array(Some(&vao));
                let flat_vertices : Vec<f32> = vertices.iter().flat_map(|x| *x).collect();
                let vertex = self.create_buffer_f32(flat_vertices.as_slice());
                
                match vertex {
                    Err(x) => Err(x),
                    Ok(vertex) => {
                        self.data.context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
                        self.data.context.enable_vertex_attrib_array(0);

                        let flat_indices : Vec<u32> = indices.iter().flat_map(|x| *x).collect();
                        self.create_buffer_u32(flat_indices.as_slice())
                            .map(|index| MeshWGL { id: vao, vertices: vertex, indices: index, vertex_count: vertices.len(), index_count: indices.len() })
                    }
                }
            }
        }
    }

    fn draw_mesh (&self, mesh: &MeshWGL) {
        self.data.context.bind_vertex_array(Some(&mesh.id));
        self.data.context.enable_vertex_attrib_array(0);

        self.data.context.draw_elements_with_i32(if self.wireframe { WebGl2RenderingContext::LINES } else { WebGl2RenderingContext::TRIANGLES }, 3 * mesh.get_index_count() as i32, WebGl2RenderingContext::UNSIGNED_INT, 0);
        
        self.data.context.disable_vertex_attrib_array(0);
        self.data.context.bind_vertex_array(None);
    }

    fn create_texture (&self, size: (u32, u32), bytes: Vec<u8>) -> Result<WebGlTexture, JsValue> {
        let id = self.data.context.create_texture();
        self.data.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, id.as_ref());

        match id {
            None => Err(JsValue::from_str("Error generating texture")),
            Some(id) => {
                let clamped : Clamped<&[u8]> = Clamped(bytes.as_slice());
                let data : Result<(), JsValue> = ImageData::new_with_u8_clamped_array_and_sh(clamped, size.0, size.1)
                    .flat_map_single(|data| {
                        self.data.context.tex_image_2d_with_u32_and_u32_and_image_data(
                            WebGl2RenderingContext::TEXTURE_2D,
                            0,
                            WebGl2RenderingContext::RGBA as i32,
                            WebGl2RenderingContext::RGBA,
                            WebGl2RenderingContext::UNSIGNED_BYTE,
                            &data
                        )
                    }
                );
        
                match data {
                    Err(x) => Err(x),
                    Ok(_) => {
                        if size.0.is_power_of_two() && size.1.is_power_of_two() {
                            self.data.context.generate_mipmap(WebGl2RenderingContext::RGBA);
                        } else {
                            self.data.context.tex_parameteri(
                                WebGl2RenderingContext::TEXTURE_2D,
                                WebGl2RenderingContext::TEXTURE_WRAP_S,
                                WebGl2RenderingContext::CLAMP_TO_EDGE as i32
                            );
        
                            self.data.context.tex_parameteri(
                                WebGl2RenderingContext::TEXTURE_2D,
                                WebGl2RenderingContext::TEXTURE_WRAP_T,
                                WebGl2RenderingContext::CLAMP_TO_EDGE as i32
                            );
        
                            self.data.context.tex_parameteri(
                                WebGl2RenderingContext::TEXTURE_2D,
                                WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                                WebGl2RenderingContext::LINEAR as i32
                            );
                        }
        
                        Ok(id)
                    }
                }
            }
        }
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
                let mouse_listener = MouseListenerWGL(EucVecf2::default());

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
                    let scene = &mut g.game.1;
                    let size = scene.window.get_size();

                    g.game.0.data.context.viewport(0, 0, size.0 as i32, size.1 as i32);
                    g.game.0.render(scene)
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

    fn clear (&mut self) {
        self.data.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT)
    } 

    fn update (&mut self) {
        self.data.context.viewport(0, 0, self.get_width() as i32, self.get_height() as i32)
    }

    fn get_property(&self, key: &str) -> Option<Box<dyn Any>> {
        fn wrap<T: Any> (value: T) -> Option<Box<dyn Any>> {
            Some(Box::new(value))
        }

        fn wrap_catch <T: Any> (value: Result<T, JsValue>) -> Option<Box<dyn Any>> {
            match value {
                Err(x) => {
                    web_sys::console::error_1(&x);
                    None
                },
                Ok(x) => Some(Box::new(x))
            }
        }

        match key {
            "scroll_x" => wrap_catch(self.data.window.scroll_x()),
            "scroll_y" => wrap_catch(self.data.window.scroll_y()),
            "pixel_ratio" => wrap(self.data.window.device_pixel_ratio()),
            "orientation" => wrap(self.data.window.orientation()),
            _ => None
        }
    }
}

// UNFIFORM
#[derive(Debug, Clone)]
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
#[derive(Debug)]
pub struct ProgramWGL {
    data: Rc<SharedData>,
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
        self.data.context.validate_program(&self.program);

        let success = self.data.context
        .get_program_parameter(&self.program, WebGl2RenderingContext::VALIDATE_STATUS)
        .as_bool()
        .unwrap_or(false);
        
        if !success {
            let err : Option<String> = self.data.context.get_program_info_log(&self.program);
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

    fn set_bool (&mut self, key: &UniformWGL, value: bool) {
        self.set_int(key, if value { 1 } else { 0 })
    }

    fn set_int (&mut self, key: &UniformWGL, value: i32) {
        self.data.context.uniform1i(key.id.as_ref(), value)
    }

    fn set_uint (&mut self, key: &UniformWGL, value: u32) {
        self.data.context.uniform1ui(key.id.as_ref(), value)
    }

    fn set_float (&mut self, key: &UniformWGL, value: f32) {
        self.data.context.uniform1f(key.id.as_ref(), value)
    }

    fn set_double (&mut self, key: &UniformWGL, value: f64) {
        unimplemented!()
    }

    fn set_bools (&mut self, key: &UniformWGL, value: &[bool]) {
        let map : Vec<i32> = value.iter().map(|x| if *x { 1 } else { 0 }).collect();
        self.set_ints(key, map.as_ref())
    }

    fn set_ints (&mut self, key: &UniformWGL, value: &[i32]) {
        self.data.context.uniform1iv_with_i32_array(key.id.as_ref(), value)
    }

    fn set_uints (&mut self, key: &UniformWGL, value: &[u32]) {
        self.data.context.uniform1uiv_with_u32_array(key.id.as_ref(), value)
    }

    fn set_floats (&mut self, key: &UniformWGL, value: &[f32]) {
        self.data.context.uniform1fv_with_f32_array(key.id.as_ref(), value)
    }

    fn set_doubles (&mut self, key: &UniformWGL, value: &[f64]) {
        unimplemented!()
    }

    fn set_float_vec2 (&mut self, key: &Self::Uniform, value: &EucVecf2) {
        self.data.context.uniform2f(key.id.as_ref(), value.x, value.y)
    }

    fn set_float_vec3 (&mut self, key: &Self::Uniform, value: &EucVecf3) {
        self.data.context.uniform3f(key.id.as_ref(), value.x, value.y, value.z)
    }

    fn set_float_vec4 (&mut self, key: &Self::Uniform, value: &EucVecf4) {
        self.data.context.uniform4f(key.id.as_ref(), value.x, value.y, value.z, value.w)
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

    fn set_float_mat2 (&mut self, key: &Self::Uniform, value: &Matf2) {
        self.data.context.uniform_matrix2fv_with_f32_array(key.id.as_ref(), true, value.flat().as_ref())
    }

    fn set_float_mat3 (&mut self, key: &Self::Uniform, value: &Matf3) {
        self.data.context.uniform_matrix3fv_with_f32_array(key.id.as_ref(), true, value.flat().as_ref())
    }

    fn set_float_mat4 (&mut self, key: &Self::Uniform, value: &Matf4) {
        self.data.context.uniform_matrix4fv_with_f32_array(key.id.as_ref(), true, value.flat().as_ref())
    }

    fn set_double_mat2 (&mut self, key: &Self::Uniform, value: &Matd2) {
        unimplemented!()
    }

    fn set_double_mat3 (&mut self, key: &Self::Uniform, value: &Matd3) {
        unimplemented!()
    }

    fn set_double_mat4 (&mut self, key: &Self::Uniform, value: &Matd4) {
        unimplemented!()
    }
}

// SHADER
#[derive(Debug)]
pub struct VertexWGL (WebGlShader);

#[derive(Debug)]
pub struct FragmentWGL (WebGlShader);

impl VertexShader for VertexWGL {}
impl FragmentShader for FragmentWGL {}

// MESH
#[derive(Debug)]
pub struct MeshWGL {
    id: WebGlVertexArrayObject,
    vertices: WebGlBuffer,
    indices: WebGlBuffer,

    vertex_count: usize,
    index_count: usize
}

impl Mesh for MeshWGL {
    fn get_vertices<'a> (&'a self) -> &'a [EucVecf3] {
        todo!()
    }

    fn get_indices<'a> (&'a self) -> &'a [[u32;3]] {
        todo!()
    }

    fn get_normals<'a> (&'a self) -> &'a [EucVecf3] {
        todo!()
    }
    
    fn get_vertex_count (&self) -> usize {
        self.vertex_count
    }

    fn get_index_count (&self) -> usize {
        self.index_count
    }
}

// TEXTURE
// I'M NOT HAPPY WITH THIS. IN THE FUTURE, TYPE SAFETY MUST BE GUARANTEED
impl UniformValue for WebGlTexture  {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        unsafe {
            let program = &mut *(self as *const dyn Any as *mut ProgramWGL);
            let key = &*(self as *const dyn Any as *const UniformWGL);

            program.data.context.active_texture(WebGl2RenderingContext::TEXTURE0);
            program.data.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self));
            program.set_uint(key, 0);
            true
        }
    }
}

impl Texture for WebGlTexture {}

// LISTENERS
pub struct KeyboardListenerWGL ([bool;161]);

impl KeyboardListenerWGL {
    pub fn new () -> KeyboardListenerWGL {
        KeyboardListenerWGL([false; 161])
    }
}

pub struct MouseListenerWGL (EucVecf2);

impl MouseListenerWGL {
    pub fn new () -> MouseListenerWGL {
        MouseListenerWGL(EucVecf2::default())
    }
}

impl KeyboardListener for KeyboardListenerWGL {
    fn is_pressed (&self, key: KeyboardKey) -> bool {
        self.0[key as usize]
    }

    fn init () -> Self {
        todo!()
    }
}

impl MouseListener for MouseListenerWGL {
    fn relative_position (&self) -> EucVecf2{
        self.0.clone()
    }

    fn init () -> Self {
        Self::new()
    }
}