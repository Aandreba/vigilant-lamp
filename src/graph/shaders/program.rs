use super::shader::{FragmentShader, VertexShader};

pub trait Program {
    type Vertex: VertexShader;
    type Fragment: FragmentShader;

    fn get_vertex (&self) -> &Self::Vertex;
    fn get_fragment (&self) -> &Self::Fragment;

    fn bind (&self);
    fn unbind (&self);
}