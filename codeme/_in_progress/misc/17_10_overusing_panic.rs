// 17_10_overusing_panic.rs

fn main() {
    // --- The Pitfall: Overusing panic! ---
    // Relying on panic! for recoverable errors crashes the whole program.
    // Uncommenting the lines below would abort execution immediately.
    
    // let result = divide_or_panic(10.0, 0.0);
    // println!("Result: {}", result); // This code is never reached

    // --- The Fix: Using Result ---
    // By returning a Result, we let the caller decide how to handle the error.

    // Case 1: Successful operation
    let good_division = divide_with_result(10.0, 2.0);
    match good_division {
        Ok(value) => println!("10.0 / 2.0 = {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Case 2: Handled failure
    let bad_division = divide_with_result(10.0, 0.0);
    match bad_division {
        Ok(value) => println!("10.0 / 0.0 = {}", value),
        Err(e) => println!("Error during division: {}", e), // Logic flows gracefully
    }
}

// Bad Practice: Crashes the program on error
fn divide_or_panic(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        panic!("Critical error: Division by zero!"); 
    }
    numerator / denominator
}

// Good Practice: Returns a Result<T, E>
fn divide_with_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}