use crate::math::array_ext::NumArray;

pub struct Mesh {
    vertices: Vec<NumArray<f32, 3>>,
    indices: Vec<[u32; 3]>
}

// IMPLEMENT
impl Mesh {
    // INIT
    pub fn new (vertices: Vec<NumArray<f32, 3>>, indices: Vec<[u32; 3]>) -> Mesh {
        Mesh { vertices, indices }
    }

    pub fn from_arrays (vertices: Vec<[f32;3]>, indices: Vec<[u32; 3]>) -> Mesh {
        Mesh { vertices: vertices.iter().map(|x| NumArray(*x)).collect() , indices }
    }
}