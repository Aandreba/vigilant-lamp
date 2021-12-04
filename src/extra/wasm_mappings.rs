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
    () => ();
    ($($arg:tt)*) => ({
        $crate::_print_args(format_args!($($arg)*));
    })
}

pub fn map_panic () {
    std::panic::set_hook(Box::new(|x| {
        /*let payload = x.payload();
        let location = x.location();

        let file: &str;
        let line: u32;
        let col: u32;
        let msg : &str;

        match location {
            Some(x) => {
                file = x.file();
                line = x.line();
                col = x.column();
            },

            None => {
                file = "";
                line = 0;
                col = 0;
            }
        }

        match payload.downcast_ref::<&str>() {
            Some(x) => msg = x,
            None => match payload.downcast_ref::<String>() {
                Some(x) => msg = x.as_str(),
                None => msg = ""
            }
        }

        let mut error = String::from_str("Error in '").unwrap();
        error.push_str(file);
        error.push_str("' at line ");
        error.push_str(line.to_string().as_str());
        error.push(':');
        error.push_str(col.to_string().as_str());
        error.push('\n');
        error.push_str(msg);

        web_sys::console::error_1(&error.as_str().into())*/
        unsafe {
            web_sys::console::error_1(&x.to_string().as_str().into())
        }
    }));
}