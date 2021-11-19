#[macro_export]
macro_rules! varray {
    [$e: expr; $len: expr] => {{
        [$e; $len];
    }};
}