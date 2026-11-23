// Function returning a Result: Ok with an f64 result, or Err with a 
// String error message.
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        // If denominator is zero, return an error (Err)
        Err(String::from("Error: division by zero!"))
    } else {
        // Otherwise, return the result successfully (Ok)
        Ok(numerator / denominator)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0); // Successful call
    let result2 = divide(5.0, 0.0); // Failing call

    // Use 'match' to handle both cases of the Result
    match result1 {
        Ok(value) => println!("Division 10/2 succeeded: {}", value), // Output: 5.0
        Err(message) => println!("Error in division 10/2: {}", message),
    }

    match result2 {
        Ok(value) => println!("Division 5/0 succeeded: {}", value),
        Err(message) => println!("Error in division 5/0: {}", message), // Output: Error: division by zero!
    }
}