use std::{fs::File, io::Read};
use crate::{engine::{input::{KeyboardListener, MouseListener}}, Scene, Texture, ErrorType, ResultFlatMap};
use super::{mesh::Mesh, shaders::{Program}, window::Window};

// RENDERER
pub trait Renderer: Sized {
   type ErrorType;
   type WindowType: Window;
   type ProgramType: Program<Error = Self::ErrorType>;
   type MeshType: Mesh;
   type TextureType: Texture;

   type KeyboardListenerType: KeyboardListener;
   type MouseListenerType: MouseListener;

   fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Result<Self::WindowType, Self::ErrorType>;
   fn create_program (&self, vertex: <Self::ProgramType as Program>::Vertex, fragment: <Self::ProgramType as Program>::Fragment, uniforms: &[&str]) -> Result<Self::ProgramType, Self::ErrorType>;
   
   fn bind_program (&self, program: &Self::ProgramType);
   fn unbind_program (&self, program: &Self::ProgramType);

   fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Result<Self::MeshType, Self::ErrorType>;
   fn draw_mesh (&self, mesh: &Self::MeshType);

   fn create_texture (&self, size: (u32, u32), bytes: Vec<u8>) -> Result<Self::TextureType, Self::ErrorType>;
   fn create_texture_from_read<R: Read> (&self, size: (u32, u32), mut image: R) -> Result<Self::TextureType, ErrorType<std::io::Error, Self::ErrorType>> {
      let mut bytes : Vec<u8> = Vec::new();

      image.read_to_end(&mut bytes)
         .flat_map(|l| self.create_texture(size, bytes))
   }

   fn create_texture_from_path (&self, size: (u32, u32), path: &str) -> Result<Self::TextureType, ErrorType<std::io::Error, Self::ErrorType>> {
      File::open(path)
         .flat_map(|file| self.create_texture_from_read(size, file))
         .map_err(|e| e.flatten())
   }

   fn create_vertex_shader (&self, code: &str) -> Result<<Self::ProgramType as Program>::Vertex, Self::ErrorType>;
   fn create_fragment_shader (&self, code: &str) -> Result<<Self::ProgramType as Program>::Fragment, Self::ErrorType>;
   
   fn create_vertex_shader_from_read<R: Read> (&self, mut code: R) -> Result<<Self::ProgramType as Program>::Vertex, Self::ErrorType> {
      let mut code_string = String::new();
      code.read_to_string(&mut code_string);
      self.create_vertex_shader(code_string.as_str())
   }

   fn create_fragment_shader_from_read<R: Read> (&self, mut code: R) -> Result<<Self::ProgramType as Program>::Fragment, Self::ErrorType> {
      let mut code_string = String::new();
      code.read_to_string(&mut code_string);
      self.create_fragment_shader(code_string.as_str())
   }

   fn create_vertex_shader_from_path (&self, path: &str) -> Result<<Self::ProgramType as Program>::Vertex, Self::ErrorType> {
      let file = File::open(path).expect("File not found");
      return self.create_vertex_shader_from_read(&file);
   }

   fn create_fragment_shader_from_path (&self, path: &str) -> Result<<Self::ProgramType as Program>::Fragment, Self::ErrorType> {
      let file = File::open(path).expect("File not found");
      return self.create_fragment_shader_from_read(&file);
   }

   fn set_wireframe (&mut self, value: bool);
   fn run (self, scene: Scene<Self>) -> Result<(), Self::ErrorType>;
}