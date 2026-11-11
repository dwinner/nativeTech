// 09_documentation_tests.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 09_documentation_tests.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

// --- Library Code ---

/// Formats a name and an age into a greeting string.
///
/// This function takes a name (a string slice) and an age (an unsigned 32-bit integer)
/// and returns a nicely formatted greeting.
///
/// # Examples
///
/// ```
/// // In a real Cargo project, you would import the crate:
/// // use string_formatter::format_greeting;
///
/// // Here, we pretend the function is available:
/// let name = "Alice";
/// let age = 30;
/// let greeting = format_greeting(name, age);
///
/// assert_eq!(greeting, "Hello, Alice! You are 30 years old.");
/// ```
///
/// You can have multiple examples. This one checks partial matches:
/// ```
/// let greeting_bob = format_greeting("Bob", 42);
/// assert!(greeting_bob.contains("Bob"));
/// assert!(greeting_bob.contains("42"));
/// ```
pub fn format_greeting(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

// --- Test Harness ---
// Since 'rustc --test' doesn't execute doc comments automatically,
// we manually create tests that mirror the documentation examples above.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_example_alice() {
        // This mirrors the first example in the documentation
        let name = "Alice";
        let age = 30;
        let greeting = format_greeting(name, age);
        assert_eq!(greeting, "Hello, Alice! You are 30 years old.");
    }

    #[test]
    fn test_doc_example_bob() {
        // This mirrors the second example in the documentation
        let greeting_bob = format_greeting("Bob", 42);
        assert!(greeting_bob.contains("Bob"));
        assert!(greeting_bob.contains("42"));
    }
}

fn main() {
    println!("This file contains documentation tests.");
    println!("Run with `rustc --test 09_documentation_tests.rs`");
}