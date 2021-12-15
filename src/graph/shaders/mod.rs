mod program;
mod shader;
mod uniform;

pub use program::{Program, Uniform};
pub use shader::{FragmentShader, VertexShader};
pub use uniform::UniformValue;

#[doc(hidden)]
pub use uniform::subkey;