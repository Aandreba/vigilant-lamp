use crate::engine::transform::Transform;
use crate::graph::{Mesh};

/// Element containing all needed information a game object
pub struct ObjectG<T: Mesh> {    
    pub mesh: T,
    pub transform: Transform
}

impl<T: Mesh> ObjectG<T> {
    pub fn new (mesh: T) -> ObjectG<T> {
        ObjectG { mesh, transform: Transform::default() }
    }
}