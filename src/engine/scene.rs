use crate::{graph::{renderer::Renderer, window::{Window}}, math::matrix::Matrix4, engine::camera};
use super::{camera::{Camera}, objectg::ObjectG, script::{Script}};

// SCENE
pub struct Scene<R: Renderer> {
    pub window: R::WindowType,
    pub program: R::ProgramType,
    pub objects: Vec<ObjectG<R::MeshType>>,
    pub camera: Box<dyn Camera>,
    pub script: Script<R>
}

impl<R: Renderer> Scene<R> {
    pub fn new<C: Camera + 'static> (window: R::WindowType, program: R::ProgramType, camera: C, objects: Vec<ObjectG<R::MeshType>>, script: Script<R>) -> Scene<R> {
        Scene { window, program, objects, camera: Box::new(camera), script }
    }

    pub fn projection_matrix (&self) -> Matrix4<f32> {
        let size = self.window.get_size();
        self.camera.projection_matrix(size.0, size.1)
    }

    pub fn camera_matrix (&self) -> Matrix4<f32> {
        self.projection_matrix() * self.camera.view_matrix()
    }
}