// 17_27_mismatched_types.rs

fn main() {
    // --- The Pitfall: Mismatched Types ---
    // Rust is statically and strongly typed. You cannot assign a string literal 
    // directly to a variable declared as an integer.
    
    // let x: i32 = "42"; 
    // Error: expected `i32`, found `&str`

    // --- The Fix: Parsing / Conversion ---
    // We must explicitly parse the string into the numeric type we need.
    
    let input = "42";

    // Option 1: Safe Parsing (Handling potential errors)
    // The `.parse()` method returns a Result.
    match input.parse::<i32>() {
        Ok(num) => {
            let x: i32 = num; // Now the types match
            println!("Successfully parsed x: {}", x);
        }
        Err(e) => {
            println!("Failed to parse '{}': {}", input, e);
        }
    }

    // Option 2: Expect (When you are certain the format is correct)
    // Useful for hardcoded values or controlled environments.
    let y: i32 = "100".parse().expect("Hardcoded string should be a valid number");
    println!("y: {}", y);
}