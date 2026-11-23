use std::fs;
use std::io;
use std::num::ParseFloatError;

// 1. Define a custom error type for our function
// Suppress the spurious 'dead_code' warning related to the derived Debug implementation
#[allow(dead_code)]
#[derive(Debug)]
enum ReadDivideError {
    Io(io::Error),
    Format(ParseFloatError),
    Math(String), 
}

// 2. Implement 'From' trait to allow '?' to automatically convert io::Error
impl From<io::Error> for ReadDivideError {
    fn from(err: io::Error) -> ReadDivideError {
        ReadDivideError::Io(err)
    }
}

// 3. Implement 'From' trait to allow '?' to automatically convert ParseFloatError
impl From<ParseFloatError> for ReadDivideError {
    fn from(err: ParseFloatError) -> ReadDivideError {
        ReadDivideError::Format(err)
    }
}

// 4. Implement 'From' trait to allow '?' to automatically convert String (from divide)
impl From<String> for ReadDivideError {
    fn from(err: String) -> ReadDivideError {
        ReadDivideError::Math(err)
    }
}

// 5. Our original 'divide' function
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(numerator / denominator)
    }
}

// Function using '?' 
fn read_then_divide_with_qmark(file_path: &str, divisor: f64) -> Result<f64, ReadDivideError> {
    // The '?' operator automatically converts and propagates errors
    let content = fs::read_to_string(file_path)?;
    let number = content.trim().parse::<f64>()?;
    let result = divide(number, divisor)?;

    Ok(result)
}

fn main() {
    // --- Setup and Tests ---
    let filename = "number.txt";
    fs::write(filename, "100.5").expect("Cannot write file");

    println!("--- 1. Success Case ---");
    match read_then_divide_with_qmark(filename, 2.0) {
        Ok(r) => println!("Result with '?': {}", r),
        Err(e) => println!("Error with '?': {:?}", e),
    }

    println!("\n--- 2. Division by Zero Error ---");
    match read_then_divide_with_qmark(filename, 0.0) {
        Ok(_) => (),
        Err(e) => println!("Error with '?': {:?}", e), 
    }

    // Clean up dummy file
    fs::remove_file(filename).ok(); 
}