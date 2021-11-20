pub trait Mesh {
    fn get_vertex_count (&self) -> usize;
    fn get_index_count (&self) -> usize;
}