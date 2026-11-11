// 17_14_uninformative_expect.rs
use std::num::ParseIntError;

fn main() {
    let valid_input = "123";
    let invalid_input = "abc";

    // --- 1. Best Approach: Return Result ---
    // Letting the caller handle the error is idiomatic and flexible.
    match parse_input_result(invalid_input) {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Handled error safely: {}", e),
    }

    // --- 2. Efficient Panic (Lazy Evaluation) ---
    // If you MUST panic, use unwrap_or_else. The closure only runs on error.
    // Uncomment to test:
    // parse_input_efficient_panic(invalid_input);

    // --- 3. The Pitfall: Uninformative expect ---
    // Panics with a generic message, hiding *which* input caused the crash.
    // Uncomment to test:
    // parse_input(invalid_input); 
    
    // --- 4. The Performance Trap: Eager Evaluation ---
    // `format!` allocates memory EVERY time, even on success. Avoid this in hot paths.
    // parse_input_inefficient_panic(valid_input); 
}

// The Pitfall: Generic panic message
fn parse_input(input_str: &str) -> u32 {
    input_str.parse().expect("Failed to parse input")
}

// The Fix: Return Result (Idiomatic)
fn parse_input_result(input_str: &str) -> Result<u32, ParseIntError> {
    input_str.parse()
}

// The Performance Trap: Eager formatting
// The `format!` macro runs and allocates a String BEFORE `expect` checks the Result.
fn parse_input_inefficient_panic(input_str: &str) -> u32 {
    input_str
        .parse()
        .expect(&format!("Failed to parse '{}' as u32", input_str))
}

// The Fix for Panics: Lazy evaluation
// The closure is only executed if `parse` returns Err.
fn parse_input_efficient_panic(input_str: &str) -> u32 {
    input_str.parse().unwrap_or_else(|e| {
        panic!("Failed to parse '{}' as u32: {}", input_str, e)
    })
}