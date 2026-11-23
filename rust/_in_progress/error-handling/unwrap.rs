// A simple function that returns a Result, for context.
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // --- Success Case ---
    let ok_result = divide(10.0, 5.0);

    // Print the Result before unwrapping to show its Ok(value)
    // state.
    println!("The Result before unwrapping: {:?}", ok_result); // Prints: Ok(2.0)

    // .unwrap() extracts the value from the Ok variant.
    let value = ok_result.unwrap();
    println!("The value after unwrap: {}", value); // Prints: 2.0

    // --- Panic Case ---
    // let err_result = divide(10.0, 0.0); // This would be
    // Err("Division by zero")

    // // Println!("The Result before panic: {:?}", err_result);
    
    // // Calling .unwrap() on an Err variant will cause the program
    // // to panic.
    // // let value_panic = err_result.unwrap(); // This line would
    // PANIC!
}