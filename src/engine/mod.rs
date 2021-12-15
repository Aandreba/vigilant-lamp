mod camera;
mod objectg;
mod scene;
mod transform;
mod clock;
mod script;
mod material;
mod color;

pub mod input;
pub mod light;

pub use camera::{Camera, PerspectiveCamera};
pub use objectg::ObjectG;
pub use scene::{Scene};
pub use transform::Transform;
pub use clock::Clock;
pub use script::Script;
pub use material::{Material, Texture};
pub use color::Color;