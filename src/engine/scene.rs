use std::time::Duration;

use crate::{graph::{renderer::Renderer, shaders::program::Program, window::Window}, math::matrix::Matrix4};
use super::{camera::Camera, clock::Clock, objectg::ObjectG};

pub struct Scene<R: Renderer, C: Camera> {
    pub renderer: R,
    pub window: R::WindowType,
    pub program: R::ProgramType,
    pub camera: C,
    pub objects: Vec<ObjectG<R::MeshType>>
}

impl<R: Renderer, C: Camera> Scene<R,C> {
    pub fn new (renderer: R, window: R::WindowType, program: R::ProgramType, camera: C, objects: Vec<ObjectG<R::MeshType>>) -> Scene<R,C> {
        Scene { renderer, window, program, camera, objects }
    }

    pub fn projection_matrix (&self) -> Matrix4<f32> {
        self.camera.projection_matrix(&self.window)
    }

    pub fn run (self) {
        self.renderer.run(self);
    }

    pub fn frame (&self, delta: Duration) {
        self.window.clear();
        self.program.bind();

        for element in self.objects.iter() {
            self.renderer.draw_mesh(&element.mesh);
        }

        self.program.unbind();
        self.window.update();
    }
}