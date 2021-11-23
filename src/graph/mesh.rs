use super::renderer::Renderer;

pub trait Mesh {
    fn get_vertex_count (&self) -> usize;
    fn get_index_count (&self) -> usize;

    fn square<R: Renderer> (renderer: &R) -> <R as Renderer>::MeshType {
        let vertices = [
            [-1., -1., 0.],
            [-1., 1., 0.],
            [1., -1., 0.],
            [1., 1., 0.],
        ];

        let indices = [
            [0, 1, 2],
            [2, 3, 1]
        ];

        renderer.create_mesh(&vertices, &indices)
    }
}