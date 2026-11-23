// 11_no_std_kernel.rs

// --- STEP-BY-STEP: HOW TO RUN THIS KERNEL MODULE ---
//
// Since this is a specialized "no_std" library, you cannot just run it in an existing bin crate.
// Follow these exact steps:
//
// 1. CREATE NEW CRATE:
//    Open your terminal and run:
//    $ cargo new --lib my_kernel_module
//    $ cd my_kernel_module
//
// 2. UPDATE Cargo.toml:
//    Open `Cargo.toml` and append these lines to handle panic/crate-type:
//
//    [lib]
//    crate-type = ["staticlib"]
//
//    [profile.dev]
//    panic = "abort"
//
//    [profile.release]
//    panic = "abort"
//
// 3. REPLACE CODE:
//    Replace the contents of `src/lib.rs` with the code below.
//
// 4. BUILD:
//    Run:
//    $ cargo build
//
//    (Success is seeing "Finished dev target(s)")

// We don't have the standard library in the kernel
#![no_std]

// Example feature for more detailed panics (requires nightly Rust)
// #![feature(panic_info_message)] 

use core::ffi::{c_char, c_int}; // For C types
use core::panic::PanicInfo;

// --- Simulate printk via FFI ---
// FIXED: `extern` blocks must be marked `unsafe` in newer Rust versions (Edition 2024+).
unsafe extern "C" {
    fn printk(fmt: *const c_char, ...) -> c_int;
}

// A safe wrapper for our simplified printk
fn kprint(message: &str) {
    // Create a fixed-size buffer for the message.
    let mut buffer = [0u8; 256];
    let mut len = 0;
    
    for (i, byte) in message.bytes().enumerate() {
        if i < buffer.len() - 1 { // Leave space for null terminator
            buffer[i] = byte;
            len = i + 1;
        } else {
            break; // Message too long for buffer
        }
    }
    
    buffer[len] = 0; // Null terminate
    
    unsafe {
        printk(buffer.as_ptr() as *const c_char);
    }
}

// --- Module Initialization and Exit ---

// FIXED: `#[no_mangle]` is unsafe in newer Rust. We use `#[unsafe(no_mangle)]`.
#[unsafe(no_mangle)]
pub extern "C" fn my_rust_module_init() -> c_int {
    kprint("Hello from Rust Kernel Module! Init function called.\n");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn my_rust_module_exit() {
    kprint("Goodbye from Rust Kernel Module! Exit function called.\n");
}

// --- Panic Handler ---

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprint("KERNEL PANIC in Rust module: ");
    
    if let Some(location) = info.location() {
        kprint("at file '");
        kprint(location.file()); 
        kprint("' line '...' "); 
    }
    
    kprint("\n");

    loop {}
}