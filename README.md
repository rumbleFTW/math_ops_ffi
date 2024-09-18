# Rust & C++ FFI Example

This repository demonstrates how to call C++ functions from Rust using the Foreign Function Interface (FFI). It showcases a simple math operations library implemented in C++ and called from Rust.

## About FFI

Foreign Function Interface (FFI) is a mechanism that allows code written in one programming language to call code written in another programming language. In this project, we use FFI to enable Rust code to call functions implemented in C++.

Key points about FFI in this context:

1. **C ABI**: FFI relies on the C Application Binary Interface (ABI) as a common ground. C++ functions need to be exposed with C linkage to be callable from Rust.

2. **Bindings Generation**: We use the `bindgen` crate to automatically generate Rust bindings for the C++ functions. This process creates safe Rust interfaces for the unsafe C++ functions.

3. **Safety**: FFI calls are inherently unsafe in Rust, as the Rust compiler cannot guarantee the safety of foreign code. Users need to use the `unsafe` keyword when calling these functions.

4. **Performance**: FFI allows for zero-cost abstractions, meaning there's typically no runtime performance penalty for calling C++ functions from Rust.

## Project Structure

- `cpp/`: Contains C++ source files and headers
- `src/`: Contains Rust source files
- `build.rs`: Rust build script for generating FFI bindings
- `CMakeLists.txt`: CMake configuration for building the C++ library
- `Cargo.toml`: Rust project configuration

## Prerequisites

- Rust (latest stable version)
- CMake (version 3.10 or higher)
- C++ compiler supporting C++14

## Building and Running

1. Generate CMake files:

```bash
cmake .
```

2. Build the C++ library:

```bash
make
```

3. Build the Rust project:

```bash
cargo build
```


## How it Works

1. The C++ library (`math_ops`) is defined in `CMakeLists.txt` and built as a static library.

2. The `build.rs` script uses `bindgen` to generate Rust bindings for the C++ functions.

3. The Rust `main.rs` file includes the generated bindings and calls the C++ functions.
