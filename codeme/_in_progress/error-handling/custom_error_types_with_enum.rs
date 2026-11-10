use std::num::ParseIntError; 
use std::io; 

// Suppress the spurious dead_code warning related to the derived Debug impl
// and the unread field warning below.
#[allow(dead_code)] 
#[derive(Debug)] 
enum DataProcessingError {
    FileNotFound(String),      
    InvalidFormat(ParseIntError), // We'll keep the error but allow the warning
    NegativeValue(i32),        
    // Removed IoError since it was never constructed in the example logic
}

// We can implement methods or traits for our error enum
impl DataProcessingError {
    fn user_message(&self) -> String {
        match self {
            DataProcessingError::FileNotFound(name) => format!(
                "Error: Could not find file '{}'.", name
            ),
            // The unread warning is here because we match on '_', ignoring the ParseIntError.
            // We keep it this way to match your desired output message.
            DataProcessingError::InvalidFormat(_) => String::from(
                "Error: Invalid data format, expected an integer."
            ),
            DataProcessingError::NegativeValue(val) => format!(
                "Error: Found negative value {}, expected positive.", val
            ),
            // Removed IoError match arm
        }
    }
}

// Example function that might return our custom error
fn process_data(data_str: &str) -> Result<i32, DataProcessingError> {
    // 1. Simulate a file not found error if input is empty (Test Case 3)
    if data_str.is_empty() {
        return Err(DataProcessingError::FileNotFound(String::from("data.txt")));
    }

    // 2. Simulate a parsing error (Test Case 2)
    let num = match data_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(DataProcessingError::InvalidFormat(e)),
    };

    // 3. Simulate a negative value error (Test Case 1)
    if num < 0 {
        return Err(DataProcessingError::NegativeValue(num));
    }

    // 4. Success case
    Ok(num)
}

fn main() {
    println!("--- Test Case 1: Negative Value Error ---");
    let result1 = process_data("-5");
    if let Err(e) = &result1 {
        println!("Error (Debug): {:?}", e);
        println!("User Message: {}", e.user_message());
    }

    println!("\n--- Test Case 2: Invalid Format Error ---");
    let result2 = process_data("abc");
    if let Err(e) = &result2 {
        println!("Error (Debug): {:?}", e);
        println!("User Message: {}", e.user_message());
    }
    
    println!("\n--- Test Case 3: File Not Found Error ---");
    let result3 = process_data("");
    if let Err(e) = &result3 {
        println!("Error (Debug): {:?}", e);
        println!("User Message: {}", e.user_message());
    }
}