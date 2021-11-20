use std::{fs::File, io::Read};

use super::{mesh::Mesh, shaders::program::Program, window::Window};

pub trait Renderer {
   type WindowType: Window;
   type ProgramType: Program;
   type MeshType: Mesh;
    
   fn create_window (&self, title: &str, width: u32, height: u32) -> Self::WindowType;
   fn create_program (&self, vertex: <Self::ProgramType as Program>::Vertex, fragment: <Self::ProgramType as Program>::Fragment) -> Self::ProgramType;
   
   fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Self::MeshType;
   fn draw_mesh (&self, mesh: Self::MeshType);

   fn create_vertex_shader<R: Read> (&self, code: R) -> <Self::ProgramType as Program>::Vertex;
   fn create_fragment_shader<R: Read> (&self, code: R) -> <Self::ProgramType as Program>::Fragment;

   fn create_vertex_shader_from (&self, path: &str) -> <Self::ProgramType as Program>::Vertex {
      let file = File::open(path).expect("File not found");
      return self.create_vertex_shader(file);
   }

   fn create_fragment_shader_from (&self, path: &str) -> <Self::ProgramType as Program>::Fragment {
      let file = File::open(path).expect("File not found");
      return self.create_fragment_shader(file);
   }
}