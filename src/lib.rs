macro_rules! export {
    ($name:ident) => {
        #[doc(hidden)]
        pub use $name::*;
    };
}

/// Math extensions to perform all operations needed
pub mod math;
export!{math}


/// Elements related to the rendering process
pub mod graph;
export!{graph}


/// Various utils
pub mod extra;
export!{extra}


/// This module contains the elements related to the engine.
/// This elements are supposed to be renderer independent, meaning that you should be able to use them in the same way, regardless of your compilation target or renderer choice
pub mod engine;
export!{engine}


/// Renderer implementations (currently, OpenGL & WASM/WebGL)
pub mod renderers;
export!{renderers}