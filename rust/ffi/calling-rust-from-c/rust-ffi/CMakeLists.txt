cmake_minimum_required(VERSION 4.2)
project(rust_ffi C)

set(CMAKE_C_STANDARD 23)

link_directories(
        "c:/gitHub/native-tech/rust/ffi/calling-rust-from-c/rust-ffi"
)

add_executable(rust_ffi main.c rust_ffi.h)

target_link_libraries(rust_ffi ffitest.dll)