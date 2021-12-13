use crate::{Material, Renderer};
use crate::engine::transform::Transform;

/// Element containing all needed information a game object
#[derive(Debug)]
pub struct ObjectG<R: Renderer> {    
    pub mesh: R::MeshType,
    pub transform: Transform,
    pub material: Material<R>
}

impl<R: Renderer> ObjectG<R> {
    pub fn new (mesh: R::MeshType, transform: Transform, material: Material<R>) -> ObjectG<R> {
        ObjectG { mesh, transform, material }
    }

    pub fn of_mesh (mesh: R::MeshType, material: Material<R>) -> ObjectG<R> {
        ObjectG { mesh, transform: Transform::default(), material }
    }
}