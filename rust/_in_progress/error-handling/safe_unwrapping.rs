// Context function required for the example (from previous example)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    // --- 1. unwrap_or() Test ---
    // Create a new failing Result value for the first test
    let err_result_1 = divide(10.0, 0.0); 
    println!("--- 1. Using .unwrap_or() ---");
    
    // This consumes err_result_1
    let value_or_default = err_result_1.unwrap_or(0.0); 
    println!("Value from err_result_1 (or default): {}", value_or_default); 


    // --- 2. unwrap_or_else() Test ---
    // Create a new failing Result value for the second test
    let err_result_2 = divide(10.0, 0.0); 
    println!("\n--- 2. Using .unwrap_or_else() ---");
    
    // This consumes err_result_2
    let value_or_computed = err_result_2.unwrap_or_else(|err_msg| {
        println!("Error during division: {}. Using computed fallback value.",
            err_msg);
        -1.0 
    });
    println!("Value from err_result_2 (or computed): {}", value_or_computed); 
}