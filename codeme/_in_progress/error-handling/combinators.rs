use std::num::ParseIntError; // Needed to specify the error type for parse()

// A function that only succeeds for even numbers. (Returns String error)
fn check_if_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err("Number is not even.".to_string())
    }
}

fn main() {
    // Both parse operations use ParseIntError as the error type E
    let successful_parse: Result<i32, ParseIntError> = "10".parse();
    let failed_parse: Result<i32, ParseIntError> = "7".parse();

    // The fix: Use .map_err() to convert the ParseIntError (E) into a String,
    // so both Results in the chain have the same Err type (String).

    // 1. Successful chain
    let even_result = successful_parse
        .map_err(|e| e.to_string()) // Convert ParseIntError to String
        .and_then(check_if_even);
    println!("Result for '10': {:?}", even_result); 

    // 2. Failure chain
    let odd_result = failed_parse
        .map_err(|e| e.to_string()) // Convert ParseIntError to String
        .and_then(check_if_even);
    println!("Result for '7': {:?}", odd_result); 
}