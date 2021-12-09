use vigilant_lamp::{wasm, desktop};

wasm! {
    include!("wasm.rs");
}

desktop! {
    include!("main.rs");
}