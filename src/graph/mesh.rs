use std::{f32::consts::PI, ops::Add};
use num::iter::Range;

use super::renderer::Renderer;
use crate::extra::array_builder::ArrayBuilder;

const SQUARE_VERTICES : [[f32;3];4] = [
    [-1., -1., 0.],
    [-1., 1., 0.],
    [1., -1., 0.],
    [1., 1., 0.],
];

const SQUARE_INDICES : [[u32;3];2] = [
    [0, 1, 2],
    [2, 3, 1]
];

pub trait Mesh {
    fn get_vertex_count (&self) -> usize;
    fn get_index_count (&self) -> usize;
}

pub struct MeshPrimitives ();

impl MeshPrimitives {
    pub fn square<R: Renderer> (renderer: &R) -> <R as Renderer>::MeshType {
        renderer.create_mesh(&SQUARE_VERTICES, &SQUARE_INDICES)
    }

    pub fn circle<R: Renderer, const S: usize> (renderer: &R) -> <R as Renderer>::MeshType {
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

        renderer.create_mesh(&vertices, &indices)
    }
}