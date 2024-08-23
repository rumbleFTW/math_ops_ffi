cmake_minimum_required(VERSION 3.10)
project(RustCppFFI VERSION 1.0)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Set the output directory for the library
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)

# Add the C++ library
add_library(math_ops STATIC
    cpp/src/math_ops.cpp
)

# Include directories
target_include_directories(math_ops PUBLIC
    ${CMAKE_SOURCE_DIR}/cpp/include
)

# Compiler flags
target_compile_options(math_ops PRIVATE -Wall -Wextra)