use std::{any::Any, collections::HashMap, pin::Pin, ops::Deref, rc::Rc, sync::{Arc, Mutex}, borrow::BorrowMut};
use rayon::{range, iter::{IntoParallelIterator, ParallelIterator}};
use crate::{Color, Renderer, Window, shaders::{Uniform, Program, VertexShader, FragmentShader, UniformValue}, Mesh, Texture, input::{KeyboardListener, MouseListener}, vector::{EucVecd3, EucVecf3}, Material, Minmax};

// CANVAS
pub trait CanvasCPU: Default + Sync + Send {
    fn get_width (&self) -> u32;
    fn get_height (&self) -> u32;
    fn set_pixel (&mut self, x: u32, y: u32, color: Color);

    fn get_size (&self) -> (u32, u32) {
        (self.get_width(), self.get_height())
    }
}

pub trait DrawCanvas {
    fn fill_rectangle (self, a: (u32, u32), b: (u32, u32), color: Color);
    fn fill_triangle (self, a: (u32, u32), b: (u32, u32), c: (u32, u32), color: Color);
}

impl<C: CanvasCPU> DrawCanvas for Arc<Mutex<C>> {
    fn fill_rectangle (self, a: (u32, u32), b: (u32, u32), color: Color) {
            ((a.0..b.0)).into_par_iter().for_each(|x| {
                let mut lock = self.lock().unwrap();
                lock.set_pixel(x, 1, color)
            })
        }

    fn fill_triangle (self, a: (u32, u32), b: (u32, u32), c: (u32, u32), color: Color) {
        fn dist (from: &(u32, u32), to: &(u32, u32)) -> f32 {
            let x = to.0.abs_diff(from.0) as f32;
            let y = to.1.abs_diff(from.1) as f32;
            x.hypot(y)
        }

        fn area (a: f32, b: f32, c: f32) -> f32 {
            let s = (a + b + c) / 2.;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
        
        let ab = dist(&a, &b);
        let bc = dist(&b, &c);
        let ca = dist(&c, &a);
        let abc = area(ab, bc, ca);

        let from = (a.0.min(b.0.min(c.0)), a.1.min(b.1.min(c.1)));
        let to = (a.0.max(b.0.max(c.0)), a.1.max(b.1.max(c.1)));

        (from.0..to.0).into_par_iter().for_each(|x| {
            (from.1..to.1).into_par_iter().for_each(|y| {
                let slice = (x, y);
                let pa = dist(&slice, &a);
                let pb = dist(&slice, &b);
                let pc = dist(&slice, &c);

                let pab = area(pa, ab, ab);
                let pbc = area(pb, bc, pc);
                let pac = area(pa, ca, pc);

                if pab + pbc + pac == abc {
                    let mut lock = self.lock().unwrap();
                    lock.set_pixel(x, y, color)
                }
            })
        })
    }
}

// RENDERER / WINDOW
pub struct WindowCPU<C: CanvasCPU + Send> {
    title: String,
    width: u32,
    height: u32,
    renderer: Option<ProgramCPU<C>>,

    front: Arc<Mutex<C>>,
    back: Arc<Mutex<C>>,
}

impl<C: CanvasCPU + Send> WindowCPU<C> {
    pub fn new (title: &str, width: u32, height: u32) -> WindowCPU<C> {
        WindowCPU { title: title.to_string(), width, height, renderer: None, front: Arc::new(Mutex::new(C::default())), back: Arc::new(Mutex::new(C::default())) }
    }
}

impl<C: CanvasCPU + Send> Renderer for WindowCPU<C> {
    type ErrorType = String;
    type WindowType = Self;
    type ProgramType = ProgramCPU<C>;
    type MeshType = MeshCPU;
    type TextureType = TextureCPU;
    type KeyboardListenerType = KeyboardCPU;
    type MouseListenerType = MouseCPU;

    fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Result<Self::WindowType, Self::ErrorType> {
        unimplemented!()
    }

    fn create_program (&self, vertex: <ProgramCPU<C> as Program>::Vertex, fragment: <ProgramCPU<C> as Program>::Fragment, uniforms: &[&str]) -> Result<Self::ProgramType, Self::ErrorType> {
        let uniforms = uniforms.iter()
            .map(|x| (x.to_string(), None))
            .collect();

        Ok(ProgramCPU {
            uniforms,
            vertex,
            fragment
        })
    }

    fn bind_program (&mut self, program: &Self::ProgramType) {
        todo!()
        //self.renderer = Some(program);
    }

    fn unbind_program (&mut self, program: &Self::ProgramType) {
        self.renderer = None;
    }

    fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Result<Self::MeshType, Self::ErrorType> {
        Ok(MeshCPU {
            vertices: vertices.to_owned(),
            indices: indices.to_owned()
        })
    }

    fn draw_mesh (&self, mesh: &Self::MeshType) {
        todo!()
    }

    fn create_texture (&self, size: (u32, u32), bytes: Vec<u8>) -> Result<Self::TextureType, Self::ErrorType> {
        todo!()
    }

    fn create_vertex_shader (&self, code: &str) -> Result<<ProgramCPU<C> as Program>::Vertex, Self::ErrorType> {
        todo!()
    }

    fn create_fragment_shader (&self, code: &str) -> Result<<ProgramCPU<C> as Program>::Fragment, Self::ErrorType> {
        todo!()
    }

    fn set_wireframe (&mut self, value: bool) {
        todo!()
    }

    fn run (self, scene: crate::Scene<Self>) -> Result<(), Self::ErrorType> {
        todo!()
    }
}

impl<C: CanvasCPU + Send> Window for WindowCPU<C> {
    fn get_title (&self) -> &str {
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

    fn clear (&mut self) {
        self.front.clone().fill_rectangle((0,0), self.get_size(), Color::TRANSPARENT);
    }

    fn update (&mut self) {
        let holder = self.back.clone();
        self.front = self.back.clone();
        self.back = holder
    }

    fn get_property (&self, key: &str) -> Option<Box<dyn Any>> {
        None
    }
}

// SHADERS
pub type VertexCPU = Box<dyn Fn(EucVecf3) -> EucVecf3>;
pub type FragmentCPU<C: CanvasCPU> = Box<dyn Fn(Material<WindowCPU<C>>) -> Color>;

impl VertexShader for VertexCPU {

}

impl<C: CanvasCPU + Send> FragmentShader for FragmentCPU<C> {

}

// PROGRAM
pub struct ProgramCPU<C: CanvasCPU + Send> {
    uniforms: HashMap<String, Option<Box<dyn UniformValue>>>,
    vertex: VertexCPU,
    fragment: FragmentCPU<C>
}

impl<C: CanvasCPU + Send> Program for ProgramCPU<C> {
    type Error = String;
    type Vertex = Box<dyn Fn(EucVecf3) -> EucVecf3>;
    type Fragment = Box<dyn Fn(Material<WindowCPU<C>>) -> Color>;
    type Uniform = String;

    fn get_vertex (&self) -> &Self::Vertex {
        &self.vertex
    }

    fn get_fragment (&self) -> &Self::Fragment {
        &self.fragment
    }

    fn validate (&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn get_uniforms (&self) -> &[String] {
        todo!()
    }

    fn get_uniform (&self, name: &str) -> Option<&Self::Uniform> {
        todo!()
    }

    fn set_bool (&mut self, key: &Self::Uniform, value: bool) {
        self.uniforms.insert(key.clone(), Some(Box::new(value)));
    }

    fn set_int (&mut self, key: &Self::Uniform, value: i32) {
        self.uniforms.insert(key.clone(), Some(Box::new(value)));
    }

    fn set_uint (&mut self, key: &Self::Uniform, value: u32) {
        self.uniforms.insert(key.clone(), Some(Box::new(value)));
    }

    fn set_float (&mut self, key: &Self::Uniform, value: f32) {
        self.uniforms.insert(key.clone(), Some(Box::new(value)));
    }

    fn set_double (&mut self, key: &Self::Uniform, value: f64) {
        self.uniforms.insert(key.clone(), Some(Box::new(value)));
    }

    fn set_bools (&mut self, key: &Self::Uniform, value: &[bool]) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.to_owned())));
    }

    fn set_ints (&mut self, key: &Self::Uniform, value: &[i32]) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.to_owned())));
    }

    fn set_uints (&mut self, key: &Self::Uniform, value: &[u32]) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.to_owned())));
    }

    fn set_floats (&mut self, key: &Self::Uniform, value: &[f32]) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.to_owned())));
    }

    fn set_doubles (&mut self, key: &Self::Uniform, value: &[f64]) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.to_owned())));
    }

    fn set_float_vec2 (&mut self, key: &Self::Uniform, value: &crate::vector::EucVecf2) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_float_vec3 (&mut self, key: &Self::Uniform, value: &EucVecf3) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_float_vec4 (&mut self, key: &Self::Uniform, value: &crate::vector::EucVecf4) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_float_mat2 (&mut self, key: &Self::Uniform, value: &crate::matrix::Matf2) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_float_mat3 (&mut self, key: &Self::Uniform, value: &crate::matrix::Matf3) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_float_mat4 (&mut self, key: &Self::Uniform, value: &crate::matrix::Matf4) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_double_vec2 (&mut self, key: &Self::Uniform, value: &crate::vector::EucVecd2) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_double_vec3 (&mut self, key: &Self::Uniform, value: &EucVecd3) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_double_vec4 (&mut self, key: &Self::Uniform, value: &crate::vector::EucVecd4) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_double_mat2 (&mut self, key: &Self::Uniform, value: &crate::matrix::Matd2) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_double_mat3 (&mut self, key: &Self::Uniform, value: &crate::matrix::Matd3) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }

    fn set_double_mat4 (&mut self, key: &Self::Uniform, value: &crate::matrix::Matd4) {
        self.uniforms.insert(key.clone(), Some(Box::new(value.clone())));
    }
}

// UNIFORM
impl Uniform for String {
    fn get_name (&self) -> &str {
        self.as_str()
    }
}

// MESH
pub struct MeshCPU {
    vertices: Vec<[f32;3]>,
    indices: Vec<[u32;3]>
}

impl Mesh for MeshCPU {
    fn get_vertex_count (&self) -> usize {
        self.vertices.len()
    }

    fn get_index_count (&self) -> usize {
        self.indices.len()
    }
}

// TEXTURE
#[derive(Clone)]
pub struct TextureCPU ();

impl UniformValue for TextureCPU {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool where Self: Sized {
        todo!()
    }
}

impl Texture for TextureCPU {}

// LISTENERS
pub struct KeyboardCPU ();
pub struct MouseCPU ();

impl KeyboardListener for KeyboardCPU {
    fn init () -> Self {
        todo!()
    }

    fn is_pressed (&self, key: crate::input::KeyboardKey) -> bool {
        todo!()
    }
}

impl MouseListener for MouseCPU {
    fn init () -> Self {
        todo!()
    }

    fn relative_position (&self) -> crate::vector::EucVecf2 {
        todo!()
    }
}