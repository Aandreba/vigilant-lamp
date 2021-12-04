use std::ops::DerefMut;
use wasm_bindgen::{convert::{IntoWasmAbi, FromWasmAbi, WasmSlice}, JsValue};

pub trait Wasmable<T: IntoWasmAbi> {
    fn get_wasmable (self) -> T;
}

impl <T: IntoWasmAbi> Wasmable<T> for T {
    fn get_wasmable (self) -> Self {
        self
    }
}