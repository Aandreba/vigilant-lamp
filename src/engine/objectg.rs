use crate::engine::transform::Transform;
use crate::graph::mesh::{Mesh};

pub struct ObjectG<T: Mesh> {    
    pub mesh: T,
    pub transform: Transform
}

impl<T: Mesh> ObjectG<T> {
    pub fn new (mesh: T) -> ObjectG<T> {
        ObjectG { mesh, transform: Transform::default() }
    }
}