use std::{alloc::Layout, ptr::addr_of};

pub unsafe fn malloc_ptr <T> () -> *const T {
    let layout = Layout::new::<T>();
    std::alloc::alloc(layout) as *const T
}

pub unsafe fn malloc_mut_ptr <T> () -> *mut T {
    let layout = Layout::new::<T>();
    std::alloc::alloc(layout) as *mut T
}

pub unsafe fn malloc<T: Copy> () -> T {
    let ptr = malloc_ptr();
    *ptr
}

pub unsafe fn malloc_array <T> (len: usize) -> *mut u8 {
    let layout = Layout::from_size_align(std::mem::size_of::<T>() * len, std::mem::align_of::<T>()).unwrap();
    std::alloc::alloc(layout)
}

pub unsafe fn malloc_slice <'a, T> (len: usize) -> &'a [T] {
    let ptr = malloc_array::<T>(len);
    std::slice::from_raw_parts(ptr as *const T, len)
}

pub unsafe fn malloc_mut_slice <'a, T> (len: usize) -> &'a mut [T] {
    let ptr = malloc_array::<T>(len);
    std::slice::from_raw_parts_mut(ptr as *mut T, len)
}

// USE WITH EXTREME CAUTION
pub unsafe fn cast_unchecked <I: Copy, O: Copy> (val: I) -> O {
    *(addr_of!(val) as *const O)
}

pub fn map_slice<'a, I: Clone, O, F: Fn(I) -> O> (slice: &'a [I], map: F) -> &'a [O] {
    let len = slice.len();
    let result;

    unsafe { result = malloc_mut_slice::<O>(len); }

    for i in 0..len {
        result[i] = map(slice[i].clone());
    }

    result
}