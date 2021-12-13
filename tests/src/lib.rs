use vigilant_lamp::{wasm, desktop};

wasm! {
    include!("main.rs");
}

desktop! {
    include!("desktop.rs");
}