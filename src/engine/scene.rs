use crate::{graph::window::Window, math::matrix::Matrix4};
use super::{camera::Camera, objectg::ObjectG};

pub struct Scene<W: Window, C: Camera> {
    pub window: W,
    pub camera: C,
    pub objects: Vec<ObjectG>
}

impl<W: Window, C: Camera> Scene<W,C> {
    pub fn new (window: W, camera: C) -> Scene<W,C> {
        Scene { window, camera, objects: Vec::new() }
    }

    pub fn view_matrix (&self) -> Matrix4<f32> {
        self.camera.view_matrix(&self.window)
    }
}