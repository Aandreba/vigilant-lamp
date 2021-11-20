use crate::{graph::{renderer::Renderer, window::Window}, math::matrix::Matrix4};
use super::{camera::Camera, objectg::ObjectG};

pub struct Scene<R: Renderer, C: Camera> {
    pub window: R::WindowType,
    pub program: R::ProgramType,
    pub camera: C,
    pub objects: Vec<ObjectG<R::MeshType>>
}

impl<R: Renderer, C: Camera> Scene<R,C> {
    pub fn new (window: R::WindowType, program: R::ProgramType, camera: C, objects: Vec<ObjectG<R::MeshType>>) -> Scene<R,C> {
        Scene { window, program, camera, objects }
    }

    pub fn projection_matrix (&self) -> Matrix4<f32> {
        self.camera.projection_matrix(&self.window)
    }

    pub fn update (&self) {
        self.window.update();
    }
}