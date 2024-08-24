use math_ops_ffi::{add_cpp, subtract_cpp};

fn main() {
    let result_add = add_cpp(5, 3);
    println!("5 + 3 = {}", result_add);

    let result_subtract = subtract_cpp(10, 4);
    println!("10 - 4 = {}", result_subtract);
}
