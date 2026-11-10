// 06_build.rs
// IMPORTANT: To use this with Cargo, you must rename this file to "build.rs"
// and place it in the root of your project (next to Cargo.toml).

fn main() {
    // Get the current directory (where Cargo.toml and build.rs are)
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Tell Cargo to link against our C library.
    // Assumes libmy_c_lib.a (or .lib) is in the project root.
    // If it's in a subdirectory like "clib", use:
    // println!("cargo:rustc-link-search=native={}/clib", project_dir);
    println!("cargo:rustc-link-search=native={}", project_dir); // Search in project root

    // Link against the static library "my_c_lib"
    // Cargo will look for libmy_c_lib.a on Unix-like systems
    // or my_c_lib.lib on Windows.
    println!("cargo:rustc-link-lib=static=my_c_lib");

    // If your C library had other dependencies, you'd link them here too.
    // e.g., println!("cargo:rustc-link-lib=dylib=some_other_system_lib");

    // Tell Cargo to rerun this build script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    // Also, if your C library source changes, you might want to recompile it here
    // using the `cc` crate if you weren't pre-compiling it manually.
}