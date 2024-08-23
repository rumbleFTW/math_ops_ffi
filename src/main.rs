use math_ops_ffi::{rust_add, rust_subtract};

fn main() {
    let result_add = rust_add(5, 3);
    println!("5 + 3 = {}", result_add);

    let result_subtract = rust_subtract(10, 4);
    println!("10 - 4 = {}", result_subtract);
}
