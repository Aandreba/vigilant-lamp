#[cfg(not(target_family = "wasm"))]
pub mod opengl;

#[cfg(target_family = "wasm")]
pub mod webgl;