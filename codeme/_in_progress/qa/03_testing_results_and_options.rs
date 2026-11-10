// 03_testing_results_and_options.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 03_testing_results_and_options.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

// A function that might fail if input is empty.
// Returns Result<SuccessType, ErrorType>
fn create_greeting(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(format!("Hello, {}!", name))
    }
}

// A function that returns an Option.
// Returns Some(value) or None
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // Return the first even number found
        }
    }
    None // No even number found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_greeting_success() {
        let result = create_greeting("Rustacean");
        
        // Check if the Result is the Ok variant
        assert!(result.is_ok(), "Greeting should be Ok for valid name");
        
        // Once we know it's Ok, it's generally safe to unwrap in a test.
        // If unwrap() fails, it panics, which causes the test to fail (which is what we want).
        assert_eq!(result.unwrap(), "Hello, Rustacean!");
    }

    #[test]
    fn test_create_greeting_failure_empty_name() {
        let result = create_greeting("");
        
        // Check if the Result is the Err variant
        assert!(result.is_err(), "Greeting should be Err for empty name");
        
        // We can also extract and check the specific error message
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_create_greeting_failure_whitespace_name() {
        let result = create_greeting(" ");
        assert!(result.is_err(), "Greeting should be Err for whitespace-only name");
    }

    #[test]
    fn test_find_even_number_some_found() {
        let numbers = [1, 3, 4, 5, 7];
        let result = find_even_number(&numbers);
        
        // Check if the Option is Some
        assert!(result.is_some(), "Should find an even number");
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_find_even_number_none_found() {
        let numbers = [1, 3, 5, 7, 9];
        let result = find_even_number(&numbers);
        
        // Check if the Option is None
        assert!(result.is_none(), "Should return None if no even number is present");
    }

    #[test]
    fn test_find_even_number_empty_slice() {
        let numbers: [i32; 0] = []; // Empty slice
        let result = find_even_number(&numbers);
        assert!(result.is_none(), "Should return None for an empty slice");
    }

    // Using assert!(matches!(...)) for more precise checks.
    // This is often cleaner than checking is_ok() and then unwrap().
    #[test]
    fn test_greeting_with_matches_macro() {
        let result_ok = create_greeting("Pat");
        
        // Check that it is Ok AND the string inside is "Hello, Pat!"
        assert!(
            matches!(result_ok, Ok(ref s) if s == "Hello, Pat!"), 
            "Expected Ok(\"Hello, Pat!\"), got {:?}", result_ok
        );

        let result_err = create_greeting(" ");
        
        // Check that it is Err AND the error string is specific
        assert!(
            matches!(result_err, Err(ref s) if s == "Name cannot be empty"), 
            "Expected specific error, got {:?}", result_err
        );
    }
}

fn main() {
    println!("This file contains unit tests.");
    println!("Run with `rustc --test 03_testing_results_and_options.rs`");
}