use std::fs;

// A simple 'divide' function required for the main logic.
// Returns an error as a String.
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Verbose example without '?'
// This function manually propagates errors, mapping all specific error types 
// (IO, Parse, Division) into the function's return error type (String).
fn read_then_divide(file_path: &str, divisor: f64) -> Result<f64, String> {
    // 1. File Reading Step: Error type is std::io::Error
    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(c) => c,
        // Manual propagation: Converts std::io::Error (e) to a String and returns.
        Err(e) => return Err(format!("File read error: {}", e)),
    };
    
    // 2. Number Parsing Step: Error type is std::num::ParseFloatError
    let number_result = content.trim().parse::<f64>();
    let number = match number_result {
        Ok(n) => n,
        // Manual propagation: Converts ParseFloatError (e) to a String and returns.
        Err(e) => return Err(format!("Number parse error: {}", e)),
    };

    // 3. Division Step: Error type is already String
    match divide(number, divisor) {
        Ok(result) => Ok(result),
        Err(e) => Err(e), // Simple propagation (since error type already matches String)
    }
}

fn main() {
    // --- Setup: Create a file with valid content ---
    let filename = "number.txt";
    fs::write(filename, "200").expect("Failed to write test file!");
    
    // Test Case 1: Success
    match read_then_divide(filename, 5.0) {
        Ok(val) => println!("SUCCESS: Final result is {}", val), // Output: 40
        Err(e) => println!("FAILURE: {}", e),
    }
    
    // Test Case 2: Manual Propagation of File Read Error
    match read_then_divide("non_existent.txt", 1.0) {
        Ok(_) => (),
        Err(e) => println!("\nFAILURE (Propagation 1): {}", e), // Output: File read error
    }
    
    // Test Case 3: Manual Propagation of Parse Error
    fs::write(filename, "not_a_number").expect("Failed to overwrite test file!");
    match read_then_divide(filename, 1.0) {
        Ok(_) => (),
        Err(e) => println!("\nFAILURE (Propagation 2): {}", e), // Output: Number parse error
    }
    
    // Clean up
    fs::remove_file(filename).expect("Failed to remove test file!");
}