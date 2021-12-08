use std::{io, str::FromStr, fmt::{self, Write}, ffi::CString, rc::Rc};
use wasm_bindgen::{JsValue, convert::IntoWasmAbi, describe::WasmDescribe};

#[doc(hidden)]
pub fn _print_args(args: fmt::Arguments) {
    let mut buf = String::new();
    buf.write_fmt(args);

    unsafe {
        web_sys::console::log_1(&buf.as_str().into())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print_args(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ({web_sys::console::log_0()});
    ($($arg:tt)*) => ({
        use crate::extra::wasm_mappings::_print_args;
        _print_args(format_args!($($arg)*));
    })
}

pub fn map_panic () {
    std::panic::set_hook(Box::new(|x| {
        unsafe {
            web_sys::console::error_1(&x.to_string().as_str().into())
        }
    }));
}