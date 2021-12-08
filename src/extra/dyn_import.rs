#[macro_export]
macro_rules! wasm {
    ($($x:item),*) => {
        $(
            #[cfg(target_family = "wasm")]
            $x
        )*
    };
}

#[macro_export]
macro_rules! desktop {
    ($($x:item),*) => {
        $(
            #[cfg(not(target_family = "wasm"))]
            $x
        )*
    };
}