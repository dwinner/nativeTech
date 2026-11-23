use std::fs;
use std::io;
use std::num::ParseIntError;
use std::fmt;
use std::error::Error;

// --- Custom Error Enum (Required from previous steps) ---
#[derive(Debug)]
enum DataProcessingError {
    IoError(io::Error),
    InvalidFormat(ParseIntError),
    NegativeValue(i32),
}

// Implement Display for user-friendly output ({})
impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataProcessingError::IoError(e) => write!(f, "I/O error: {}", e),
            DataProcessingError::InvalidFormat(_) => write!(f, "Invalid data format: Expected an integer"),
            DataProcessingError::NegativeValue(val) => write!(f, "Found negative value {}, expected positive.", val),
        }
    }
}

// Implement Error trait (Required for .source() and general error handling)
impl Error for DataProcessingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DataProcessingError::IoError(e) => Some(e),
            DataProcessingError::InvalidFormat(e) => Some(e),
            _ => None,
        }
    }
}

// Function that reads a number from a file, validates it's positive
fn read_positive_number(path: &str) -> Result<i32, DataProcessingError> {
    // 1. Read file content: returns Result<String, io::Error>
    let content = fs::read_to_string(path)
        // Manual error mapping required here because we haven't implemented From<io::Error>
        .map_err(|e| DataProcessingError::IoError(e))?; 

    // 2. Parse number: returns Result<i32, ParseIntError>
    let number = content.trim().parse::<i32>()
        // Manual error mapping required here because we haven't implemented From<ParseIntError>
        .map_err(|e| DataProcessingError::InvalidFormat(e))?; 

    // 3. Validate positivity (Domain Error)
    if number < 0 {
        Err(DataProcessingError::NegativeValue(number))
    } else {
        Ok(number)
    }
}

fn main() {
    // --- Test 1: Non-existent file (I/O Error) ---
    println!("--- Test 1: I/O Error (File Not Found) ---");
    match read_positive_number("non_existent_data.txt") {
        Ok(n) => println!("Number read: {}", n),
        // Uses Display impl for easy output
        Err(e) => println!("Error: {}", e), 
    }
    
    // --- Test 2: Invalid Format Error ---
    let invalid_data_path = "invalid_data.txt";
    fs::write(invalid_data_path, "abc").unwrap();
    println!("\n--- Test 2: Invalid Format Error ---");
    match read_positive_number(invalid_data_path) {
        Ok(n) => println!("Number read: {}", n),
        // Output: Error: Invalid data format: Expected an integer
        Err(e) => println!("Error: {}", e), 
    }
    fs::remove_file(invalid_data_path).ok();
    
    // --- Test 3: Negative Value Error (Domain Error) ---
    let negative_data_path = "negative_data.txt";
    fs::write(negative_data_path, "-10").unwrap();
    println!("\n--- Test 3: Negative Value Error ---");
    match read_positive_number(negative_data_path) {
        Ok(n) => println!("Number read: {}", n),
        // Output: Error: Found negative value -10, expected positive.
        Err(e) => println!("Error: {}", e), 
    }
    fs::remove_file(negative_data_path).ok();
    
    // --- Test 4: Success ---
    let valid_data_path = "valid_data.txt";
    fs::write(valid_data_path, "123").unwrap();
    println!("\n--- Test 4: Success ---");
    match read_positive_number(valid_data_path) {
        Ok(n) => println!("Number read: {}", n), // Output: Number read: 123
        Err(e) => println!("Error: {}", e), 
    }
    fs::remove_file(valid_data_path).ok();
}