// 02_assertion_macros.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 02_assertion_macros.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

#[cfg(test)]
mod tests {
    
    // 1. assert!(condition, message)
    // Checks that a boolean condition is true.
    // Useful for general validation or checking return values that are booleans.
    #[test]
    fn is_string_long_enough() {
        let my_string = "Rustacean";
        // If the condition is false, the custom message is printed.
        assert!(
            my_string.len() > 5, 
            "String '{}' should be longer than 5 chars", 
            my_string
        );
    }

    // 2. assert_eq!(left, right, message)
    // Checks that left == right.
    // Most common assertion. Prints both values if they differ.
    // Types must implement PartialEq and Debug (to print them).
    #[test]
    fn values_should_be_equal() {
        let calculated_value = 2 + 2;
        let expected_value = 4;
        assert_eq!(
            calculated_value, 
            expected_value, 
            "Checking simple addition"
        );
    }

    // 3. assert_ne!(left, right, message)
    // Checks that left != right.
    // Useful when you want to ensure a state has changed or two distinct items are not the same.
    #[test]
    fn values_should_not_be_equal() {
        let value_a = "apple";
        let value_b = "orange";
        assert_ne!(
            value_a, 
            value_b, 
            "Different fruits should not be equal"
        );
    }
}

fn main() {
    println!("This file contains unit tests.");
    println!("Run with `rustc --test 02_assertion_macros.rs`");
}