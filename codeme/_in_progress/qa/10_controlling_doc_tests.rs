// 10_controlling_doc_tests.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file:
// 1. Compile: rustc --test 10_controlling_doc_tests.rs -o test_runner
// 2. Run:     ./test_runner
//    (Note: The 'slow' test will be skipped, and the 'panic' test will pass!)
// ---------------------------------------------------------

// --- Library Code with Rich Documentation ---

/// Performs a very slow operation.
///
/// # Examples
///
/// This example is marked `ignore` because it takes too long for a quick test run.
///
/// ```ignore
/// use controlling_tests::very_slow_function;
/// very_slow_function();
/// ```
pub fn very_slow_function() {
    // Simulate work
    std::thread::sleep(std::time::Duration::from_millis(50)); 
}

/// divides two numbers.
///
/// # Examples
///
/// This example is marked `should_panic` because dividing by zero causes a crash.
/// The test runner considers this a "Success" if the code crashes as expected.
///
/// ```should_panic
/// use controlling_tests::divide;
/// divide(10, 0); // This panics!
/// ```
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

/// Starts a dangerous process (like deleting files).
///
/// # Examples
///
/// This example is marked `no_run`. The compiler checks that the code is valid syntax,
/// but it DOES NOT execute it. This is safe for side-effects.
///
/// ```no_run
/// use controlling_tests::dangerous_side_effect;
/// dangerous_side_effect(); // Compiler checks this line, but doesn't run it
/// ```
pub fn dangerous_side_effect() {
    println!("Files deleted! (Just kidding)");
}

/// Demonstrates hidden setup lines.
///
/// # Examples
///
/// Lines starting with `#` in the doc block are hidden from the user in HTML,
/// but executed by the test.
///
/// ```
/// # fn get_config() -> String { "secret".to_string() }
/// // Users only see the lines below:
/// let config = get_config();
/// assert_eq!(config, "secret");
/// ```
pub fn usage_with_hidden_lines() {
    // Logic here...
}


// --- Test Implementation ---
// These manually replicate the behavior described in the doc comments above.

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Replicating ```ignore
    #[test]
    #[ignore] // This tells the test runner to skip this function
    fn test_doc_ignore_behavior() {
        very_slow_function();
    }

    // 2. Replicating ```should_panic
    #[test]
    #[should_panic(expected = "Cannot divide by zero")] // Passes only if it crashes
    fn test_doc_should_panic_behavior() {
        divide(10, 0);
    }

    // 3. Replicating ```no_run
    // There isn't a direct #[no_run] attribute for unit tests, so we just
    // ensure it compiles. In a real doc test, the code below wouldn't execute.
    #[test]
    fn test_doc_no_run_simulation() {
        // We call it here just to prove it compiles and links.
        // In a real `no_run` doc block, the runner stops before this execution step.
        dangerous_side_effect(); 
    }

    // 4. Replicating hidden lines `#`
    #[test]
    fn test_doc_hidden_lines_simulation() {
        // The doc comment hid this function, but the test runs it.
        fn get_config() -> String { "secret".to_string() }
        
        let config = get_config();
        assert_eq!(config, "secret");
    }
}

fn main() {
    println!("This file contains documentation test examples.");
    println!("Run with `rustc --test 10_controlling_doc_tests.rs`");
}