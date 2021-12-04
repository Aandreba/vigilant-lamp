use wasm_bindgen::JsValue;
use wasm_bindgen::convert::{IntoWasmAbi, WasmAbi, WasmSlice};
use wasm_bindgen::describe::WasmDescribe;

use crate::engine::transform::Transform;
use crate::graph::mesh::{Mesh};
use crate::renderers::webgl::MeshWGL;

pub struct ObjectG<T: Mesh> {    
    pub mesh: T,
    pub transform: Transform
}

impl<T: Mesh> ObjectG<T> {
    pub fn new (mesh: T) -> ObjectG<T> {
        ObjectG { mesh, transform: Transform::default() }
    }
}