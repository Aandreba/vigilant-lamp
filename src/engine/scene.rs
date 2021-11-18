use crate::graph::window::Window;
use super::{camera::Camera, objectg::ObjectG};

pub struct Scene<W: Window, C: Camera> {
    pub window: W,
    pub camera: C,
    objects: Vec<ObjectG>
}