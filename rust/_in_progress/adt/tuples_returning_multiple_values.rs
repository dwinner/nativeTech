// The function signature indicates it returns a tuple of three i32 values.
fn calculate(a: i32, b: i32) -> (i32, i32, i32) {
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    
    // The expression below implicitly returns the tuple.
    (sum, difference, product)
}

fn main() {
    // Call the function and bind the returned tuple to the 'result' variable.
    let result = calculate(10, 5);

    println!("--- Calculation Results (10 and 5) ---");
    // Access the values using dot notation and the zero-based index.
    println!("Sum: {}", result.0); // 10 + 5 = 15
    println!("Difference: {}", result.1); // 10 - 5 = 5
    println!("Product: {}", result.2); // 10 * 5 = 50
    
    println!("\n--- Destructuring Alternative ---");
    // Alternatively, you could destructure the tuple for cleaner names:
    let (s, d, p) = calculate(20, 4);
    println!("Sum: {}, Difference: {}, Product: {}", s, d, p);
}