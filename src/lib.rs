/// Math extensions to perform all operations needed
pub mod math;
#[doc(hidden)]
pub use math::*;

/// Elements related to the rendering process
pub mod graph;
#[doc(hidden)]
pub use graph::*;

/// Various utils
pub mod extra;
#[doc(hidden)]
pub use extra::*;

/// This module contains the elements related to the engine.
/// This elements are supposed to be renderer independent, meaning that you should be able to use them in the same way, regardless of your compilation target or renderer choice
pub mod engine;
#[doc(hidden)]
pub use engine::*;

/// Renderer implementations (currently, OpenGL & WASM/WebGL)
pub mod renderers;
#[doc(hidden)]
pub use renderers::*;