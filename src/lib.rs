include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add_cpp(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}
pub fn subtract_cpp(a: i32, b: i32) -> i32 {
    unsafe { subtract(a, b) }
}
