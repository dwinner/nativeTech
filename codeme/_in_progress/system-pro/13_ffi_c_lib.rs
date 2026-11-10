// 13_ffi_c_lib.rs

use std::ffi::{CString, NulError};
use std::os::raw::{c_char, c_int};
use std::fmt;

// --- Simulated C Library (For demonstration purposes) ---
// In a real scenario, these would be in a separate .c file and linked.
// We implement them here so `cargo run` works out of the box.
#[no_mangle]
pub extern "C" fn multiply(a: c_int, b: c_int) -> c_int {
    a * b
}

#[no_mangle]
pub extern "C" fn greet_person(name: *const c_char) {
    unsafe {
        if name.is_null() {
            println!("[C Library] Received null pointer!");
            return;
        }
        let c_str = std::ffi::CStr::from_ptr(name);
        if let Ok(str_slice) = c_str.to_str() {
            println!("[C Library] Hello, {}!", str_slice);
        } else {
            println!("[C Library] Hello, (invalid UTF-8 name)!");
        }
    }
}
// --------------------------------------------------------

// Declare the C functions we want to call
extern "C" {
    fn multiply(a: c_int, b: c_int) -> c_int;
    fn greet_person(name: *const c_char);
}

// Define a custom error type for our safe wrapper module
#[derive(Debug)]
pub enum CLibError {
    StringConversion(NulError), // Error from CString::new if string has interior nulls
    // Add other potential C library error types if needed
}

impl fmt::Display for CLibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLibError::StringConversion(e) => write!(f, "Failed to convert Rust string to C string: {}", e),
        }
    }
}

impl std::error::Error for CLibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CLibError::StringConversion(e) => Some(e),
        }
    }
}

// Safe Rust wrapper functions
mod my_c_lib_wrapper {
    use super::*; // To access extern "C" block, CString, CLibError

    pub fn safe_multiply(a: i32, b: i32) -> i32 {
        // The call to the extern "C" function must be in an unsafe block.
        // We assume that passing i32s (which usually match c_int) is safe.
        unsafe {
            multiply(a as c_int, b as c_int) as i32
        }
    }

    pub fn safe_greet(name: &str) -> Result<(), CLibError> {
        // Convert Rust &str to CString (null-terminated)
        match CString::new(name) {
            Ok(c_name) => {
                // Call the C function within an unsafe block.
                // We are responsible for ensuring c_name.as_ptr() is valid.
                unsafe {
                    greet_person(c_name.as_ptr());
                }
                Ok(())
            }
            Err(e) => Err(CLibError::StringConversion(e)),
        }
    }
}

fn main() {
    println!("--- Testing FFI with Custom C Library ---");

    let num1 = 12;
    let num2 = 7;
    let product = my_c_lib_wrapper::safe_multiply(num1, num2);
    println!("Rust calling C multiply({}, {}): {}", num1, num2, product); // Expected: 84

    let name_to_greet = "Rustacean via FFI";
    match my_c_lib_wrapper::safe_greet(name_to_greet) {
        Ok(_) => println!("Greeting sent to C library successfully."),
        Err(e) => eprintln!("Error sending greeting: {}", e),
    }

    // Test with a string containing a null byte (should fail)
    println!("\n--- Testing Error Handling ---");
    let invalid_name = "Rust\0InMiddle";
    match my_c_lib_wrapper::safe_greet(invalid_name) {
        Ok(_) => println!("Unexpected success with invalid string."),
        Err(e) => println!("Caught expected error: {}", e),
    }
}