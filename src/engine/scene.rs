use std::{rc::Rc, time::Duration};
use glutin::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}};

use crate::{graph::{renderer::Renderer, window::Window}, math::matrix::Matrix4, renderers::opengl::OpenGL};
use super::{camera::{Camera}, clock::Clock, objectg::ObjectG, script::{self, Script}};

// SCENE
pub struct Scene<R: Renderer> {
    pub window: R::WindowType,
    pub program: R::ProgramType,
    pub objects: Vec<ObjectG<R::MeshType>>,
    pub camera: Box<dyn Camera>,

    pub script: Script<R>
}

impl<R: Renderer> Scene<R> {
    pub fn new<C: 'static + Camera> (window: R::WindowType, program: R::ProgramType, camera: C, objects: Vec<ObjectG<R::MeshType>>, script: Script<R>) -> Scene<R> {
        Scene { window, program, objects, camera: Box::new(camera), script: script }
    }

    fn projection_matrix (&self) -> Matrix4<f32> {
        let size = self.window.get_size();
        self.camera.projection_matrix(size.0, size.1)
    }
}