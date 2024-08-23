use std::os::raw::c_int;

#[link(name = "math_ops", kind = "static")]
extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn subtract(a: c_int, b: c_int) -> c_int;
}

pub fn rust_add(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}

pub fn rust_subtract(a: i32, b: i32) -> i32 {
    unsafe { subtract(a, b) }
}
