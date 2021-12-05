use std::{fs::File, io::Read};
use crate::engine::{input::{KeyboardListener, MouseListener}, Scene, Camera};
use super::{mesh::Mesh, shaders::{Program}, window::Window};

pub trait Renderer: Sized {
   type WindowType: Window;
   type ProgramType: Program;
   type MeshType: Mesh;

   type KeyboardListenerType: KeyboardListener;
   type MouseListenerType: MouseListener;

   fn create_window (&self, title: &str, width: u32, height: u32, vsync: bool) -> Self::WindowType;
   fn create_program (&self, vertex: <Self::ProgramType as Program>::Vertex, fragment: <Self::ProgramType as Program>::Fragment, uniforms: &[&str]) -> Self::ProgramType;
   
   fn bind_program (&self, program: &Self::ProgramType);
   fn unbind_program (&self, program: &Self::ProgramType);

   fn create_mesh (&self, vertices: &[[f32;3]], indices: &[[u32;3]]) -> Self::MeshType;
   fn draw_mesh (&self, mesh: &Self::MeshType);

   fn create_vertex_shader (&self, code: &str) -> <Self::ProgramType as Program>::Vertex;
   fn create_fragment_shader (&self, code: &str) -> <Self::ProgramType as Program>::Fragment;
   
   fn create_vertex_shader_from_read<R: Read> (&self, mut code: R) -> <Self::ProgramType as Program>::Vertex {
      let mut code_string = String::new();
      code.read_to_string(&mut code_string);
      self.create_vertex_shader(code_string.as_str())
   }

   fn create_fragment_shader_from_read<R: Read> (&self, mut code: R) -> <Self::ProgramType as Program>::Fragment {
      let mut code_string = String::new();
      code.read_to_string(&mut code_string);
      self.create_fragment_shader(code_string.as_str())
   }

   fn create_vertex_shader_from_path (&self, path: &str) -> <Self::ProgramType as Program>::Vertex {
      let file = File::open(path).expect("File not found");
      return self.create_vertex_shader_from_read(&file);
   }

   fn create_fragment_shader_from_path (&self, path: &str) -> <Self::ProgramType as Program>::Fragment {
      let file = File::open(path).expect("File not found");
      return self.create_fragment_shader_from_read(&file);
   }

   fn set_wireframe (&mut self, value: bool);
   fn run (self, scene: Scene<Self>);

   fn render (&self, scene: &mut Scene<Self>) {
      scene.window.update();
      scene.window.clear();

      self.bind_program(&scene.program);
      scene.program.set_float_mat4_by_name("camera", scene.camera_matrix());
      
      for elem in scene.objects.iter() {
         scene.program.set_float_mat4_by_name("world_matrix", elem.transform.matrix());
         self.draw_mesh(&elem.mesh)
      }

      self.unbind_program(&scene.program);
   }
}