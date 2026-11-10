use thiserror::Error;
use std::num::ParseIntError;
use std::fs;
use std::path::Path;
use std::io; // The necessary module import

// --- DEPENDENCY NOTE ---
// To use this code, you must add the 'thiserror' dependency to your Cargo.toml:
// [dependencies]
// thiserror = "2.0"
// -----------------------

#[derive(Error, Debug)]
pub enum DataProcessingError {
    
    #[error("Failed to parse content in file: {0}")]
    InvalidFormat(#[from] ParseIntError), 
    
    #[error("Value '{0}' is negative and cannot be processed")]
    NegativeValue(i32),
    
    #[error("I/O error when accessing file: {0}")]
    Io(#[from] io::Error), 
}

// Function that reads a number from a file and ensures it's not negative.
fn read_positive_number(path: &Path) -> Result<i32, DataProcessingError> {
    // The '?' operator automatically converts io::Error to DataProcessingError::Io
    let content = fs::read_to_string(path)?; 

    // The '?' operator automatically converts ParseIntError to DataProcessingError::InvalidFormat
    let number = content.trim().parse::<i32>()?; 

    // Domain validation
    if number < 0 {
        Err(DataProcessingError::NegativeValue(number))
    } else {
        Ok(number)
    }
}

fn main() {
    // Setup dummy files for demonstration
    fs::write("valid_number.txt", "123").unwrap();
    fs::write("invalid_format.txt", "abc").unwrap();
    fs::write("negative_number.txt", "-42").unwrap();

    // Test 1: File Not Found (I/O Error)
    let path_nonexistent = Path::new("non_such_file.txt");
    match read_positive_number(path_nonexistent) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e), 
    }
    
    // Test 2: Invalid Format 
    let path_invalid = Path::new("invalid_format.txt");
    match read_positive_number(path_invalid) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e), 
    }
    
    // Test 3: Negative Value (Domain Error)
    let path_negative = Path::new("negative_number.txt");
    match read_positive_number(path_negative) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e), 
    }
    
    // Test 4: Success
    let path_valid = Path::new("valid_number.txt");
    match read_positive_number(path_valid) {
        Ok(n) => println!("Successfully read positive number: {}", n),
        Err(e) => eprintln!("An unexpected error occurred: {}", e),
    }

    // Cleanup
    fs::remove_file("valid_number.txt").ok();
    fs::remove_file("invalid_format.txt").ok();
    fs::remove_file("negative_number.txt").ok();
}