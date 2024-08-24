use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=math_ops");

    let bindings = bindgen::Builder::default()
        .header("cpp/include/math_ops.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
