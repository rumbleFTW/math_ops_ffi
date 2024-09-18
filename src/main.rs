include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
fn main() {
    let result_add = unsafe { add(5, 3) };
    println!("5 + 3 = {}", result_add);

    let result_subtract = unsafe { subtract(10, 4) };
    println!("10 - 4 = {}", result_subtract);
}
