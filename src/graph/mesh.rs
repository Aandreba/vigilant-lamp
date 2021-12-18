use std::{f32::consts::PI};
use crate::{vector::EucVecf3, alloc::{malloc_slice, map_slice, cast_unchecked, malloc_mut_slice}};

use super::renderer::Renderer;

pub const SQUARE_VERTICES : [[f32;3];4] = [
    [-1., -1., 0.],
    [-1., 1., 0.],
    [1., -1., 0.],
    [1., 1., 0.],
];

pub const SQUARE_INDICES : [[u32;3];2] = [
    [0, 1, 2],
    [2, 3, 1]
];

pub const CUBE_VERTICES : [[f32;3];8] = [
    // VO
    [-1.,  1.,  1.],
    // V1
    [-1., -1.,  1.],
    // V2
    [1., -1.,  1.],
    // V3
    [1.,  1.,  1.],
    // V4
    [-1.,  1., -1.],
    // V5
    [1.,  1., -1.],
    // V6
    [-1., -1., -1.],
    // V7
    [1., -1., -1.]
];

pub const CUBE_INDICES : [[u32;3];12] = [
    // Front face
    [0, 1, 3], [3, 1, 2],
    // Top Face
    [4, 0, 3], [5, 4, 3],
    // Right face
    [3, 2, 7], [5, 3, 7],
    // Left face
    [6, 1, 0], [6, 0, 4],
    // Bottom face
    [2, 1, 6], [2, 6, 7],
    // Back face
    [7, 6, 4], [7, 4, 5]
];

pub trait Mesh {
    fn get_vertices<'a> (&'a self) -> &'a [EucVecf3];
    fn get_indices<'a> (&'a self) -> &'a [[u32;3]];
    fn get_normals<'a> (&'a self) -> &'a [EucVecf3];

    fn get_vertex_count (&self) -> usize;
    fn get_index_count (&self) -> usize;
}

type ComputedMesh<R> = Result<<R as Renderer>::MeshType, <R as Renderer>::ErrorType>;

pub struct MeshPrimitives ();

impl MeshPrimitives {
    pub fn calculate_normals<'a> (vertices: &'a [[f32;3]], indices: &'a [[u32;3]]) -> &'a [[f32;3]] {
        let vertices_cast;
        let normals;

        unsafe {
            vertices_cast = cast_unchecked::<&[[f32;3]], &[EucVecf3]>(vertices);
            normals = malloc_mut_slice::<EucVecf3>(vertices.len());
        }

        for index in indices {
            let idx0 = index[0] as usize;
            let idx1 = index[1] as usize;
            let idx2 = index[2] as usize;

            let alpha = vertices_cast[idx1] - vertices_cast[idx0];
            let beta = vertices_cast[idx2] - vertices_cast[idx0];
            let cross = alpha.cross(beta);

            normals[idx0] = normals[idx0] + cross;
            normals[idx1] = normals[idx1] + cross;
            normals[idx2] = normals[idx2] + cross;
        }

        map_slice(normals, |x| unsafe { cast_unchecked(x.unit()) })
    }

    pub fn square<R: Renderer> (renderer: &R) -> ComputedMesh<R> {
        renderer.create_mesh_wo_normals(&SQUARE_VERTICES, &SQUARE_INDICES)
    }

    pub fn cube <R: Renderer> (renderer: &R) -> ComputedMesh<R> {
        renderer.create_mesh_wo_normals(&CUBE_VERTICES, &CUBE_INDICES)
    }

    pub fn circle<R: Renderer, const S: usize> (renderer: &R) -> ComputedMesh<R> {
        let delta = 2. * PI / (S as f32);
        let mut vertices : [[f32;3];S] = [[0., 0., 0.];S];
        let mut indices : [[u32;3];S] = [[0, 0, 0];S];

        let mut i : usize = 0;
        while i < S {
            let angle = delta * (i as f32);
            let sin_cos = angle.sin_cos();

            vertices[i] = [sin_cos.1, sin_cos.0, 0.];
            indices[i] = [0, i as u32, (i + 1) as u32];
            i += 1;
        }

        renderer.create_mesh_wo_normals(&vertices, &indices)
    }
}