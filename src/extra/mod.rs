pub mod safe_zip;
pub mod iter_range;
pub mod array_concat;
pub mod array_builder;
pub mod num_two;
pub mod color;
mod error_map;

pub use error_map::{FlatMap, Flattern, ErrorType};

#[cfg(target_family = "wasm")]
pub mod wasm_mappings;