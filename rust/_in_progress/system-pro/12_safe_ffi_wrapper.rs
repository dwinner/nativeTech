// 05_safe_ffi_wrapper.rs
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::fmt;

// --- Simulated C functions ---
// In a real FFI scenario, these would be in your C library and linked externally.
// We implement them here in Rust (masquerading as C) so this file is runnable.

#[unsafe(no_mangle)] // Using unsafe(no_mangle) based on your previous compiler error
unsafe extern "C" fn simulated_c_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
unsafe extern "C" fn simulated_c_get_static_message() -> *const c_char {
    // Null-terminated byte string simulating a C static string
    static MESSAGE: &[u8] = b"Hello from simulated C!\0"; 
    MESSAGE.as_ptr() as *const c_char
}

// --- Safe Rust Wrapper Module ---
pub mod c_math_utils {
    use super::*; // To access CString, CStr, c_char, and the simulated functions

    // Define a custom error type for our wrapper
    #[derive(Debug)]
    pub enum WrapperError {
        StringConversion(std::ffi::NulError), // For CString::new failures
        Utf8Conversion(std::str::Utf8Error),  // For CStr::to_str failures
    }

    impl fmt::Display for WrapperError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                WrapperError::StringConversion(e) => write!(f, "CString conversion error: {}", e),
                WrapperError::Utf8Conversion(e) => write!(f, "CStr to &str UTF-8 conversion error: {}", e),
            }
        }
    }

    impl std::error::Error for WrapperError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                WrapperError::StringConversion(e) => Some(e),
                WrapperError::Utf8Conversion(e) => Some(e),
            }
        }
    }

    /// Safely adds two integers using the external C function.
    pub fn add_via_c(a: i32, b: i32) -> i32 {
        // The unsafe block is contained within this safe function.
        // The caller of add_via_c doesn't need to use unsafe.
        unsafe {
            // In a real scenario, we would call the extern declaration `c_add`.
            // Here we call our simulated version.
            simulated_c_add(a, b) 
        }
    }

    /// Safely gets a static message from the C library and converts it to a Rust String.
    pub fn get_message_from_c() -> Result<String, WrapperError> {
        // Unsafe block to call the C function and handle the raw pointer.
        unsafe {
            let c_char_ptr = simulated_c_get_static_message(); 
            
            if c_char_ptr.is_null() {
                // C function might return NULL to indicate an error or no message.
                // We handle this gracefully.
                return Ok(String::from("<No message from C>")); 
            }

            // Create a CStr from the raw pointer. This itself is unsafe because
            // we must trust the pointer is valid and null-terminated.
            let c_str_slice = CStr::from_ptr(c_char_ptr);

            // Attempt to convert the CStr (byte slice) to a Rust &str.
            // This can fail if the C string is not valid UTF-8.
            match c_str_slice.to_str() {
                Ok(rust_str_slice) => Ok(rust_str_slice.to_owned()), // Convert &str to owned String
                Err(e) => Err(WrapperError::Utf8Conversion(e)),
            }
        }
    }
} 

fn main() {
    println!("--- Testing Safe FFI Wrapper ---");

    // Users of our c_math_utils module call safe functions.
    // No `unsafe` block is required here!
    
    let a = 15;
    let b = 27;
    let sum = c_math_utils::add_via_c(a, b);
    println!("Safe wrapper call to add_via_c({}, {}) = {}", a, b, sum); 

    match c_math_utils::get_message_from_c() {
        Ok(message) => println!("Safe wrapper call to get_message_from_c(): '{}'", message),
        Err(e) => eprintln!("Error getting message from C via wrapper: {}", e),
    }
}