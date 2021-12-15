use crate::{graph::{Renderer, Window}, shaders::Program, Clock, input::{KeyboardListener, MouseListener}, matrix::Matf4, light::{AmbientLight, PointLight}};
use super::{camera::{Camera}, objectg::ObjectG, script::{Script}};

/// Struct containing all the inforation needed by the renderer about the contents and characteristics of the scene
pub struct Scene<R: Renderer> {
    pub window: R::WindowType,
    pub program: R::ProgramType,
    pub objects: Vec<ObjectG<R>>,
    pub camera: Box<dyn Camera>,
    pub script: Script<R>,

    pub ambient: Option<AmbientLight>,
    pub lights: Vec<PointLight>
}

impl<R: Renderer> Scene<R> {
    pub fn new<C: Camera + 'static> (window: R::WindowType, program: R::ProgramType, camera: C, objects: Vec<ObjectG<R>>, script: Script<R>) -> Scene<R> {
        Scene { window, program, objects, camera: Box::new(camera), script, ambient: None, lights: Vec::new() }
    }

    pub fn projection_matrix (&self) -> Matf4 {
        let size = self.window.get_size();
        self.camera.projection_matrix(size.0, size.1)
    }

    pub fn camera_matrix (&self) -> Matf4 {
        self.projection_matrix() * self.camera.view_matrix()
    }

    pub fn init (&mut self) -> Result<(Clock, R::KeyboardListenerType, R::MouseListenerType), R::ErrorType> {
        match self.program.validate() {
            Err(x) => Err(x),
            Ok(_) => {
                let clock = Clock::new();
                let keyboard_listener = <R::KeyboardListenerType as KeyboardListener>::init();
                let mouse_listener = <R::MouseListenerType as MouseListener>::init();

                match self.script.start {
                    Some(x) => x(self),
                    None => ()
                }

                Ok((clock, keyboard_listener, mouse_listener))
            }
        }
    }
}