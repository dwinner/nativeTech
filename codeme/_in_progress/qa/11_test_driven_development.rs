// 11_test_driven_development.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file:
// 1. Compile: rustc --test 11_test_driven_development.rs -o test_runner
// 2. Run:     ./test_runner
//
// EXPECTED OUTPUT:
// You should see several FAILED tests. This is the "Red" phase.
// ---------------------------------------------------------

// --- The Implementation (Stub) ---

// In TDD, we write the minimal amount of code to make the tests compile.
// We return `false` just to satisfy the type signature.
pub fn is_palindrome(_s: &str) -> bool {
    // TODO: Implement actual logic
    false 
    
    // Hint: To reach the "Green" phase (passing tests), replace `false` with:
    // _s.chars().eq(_s.chars().rev())
}

// --- The Tests (Specifications) ---

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test_empty_string_is_palindrome() {
        // Empty string is considered a palindrome
        assert!(is_palindrome("")); 
    }

    #[test]
    fn test_single_char_is_palindrome() {
        // Single character is always a palindrome
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_simple_palindrome() {
        // "madam" reads the same forwards and backwards
        assert!(is_palindrome("madam"));
    }

    #[test]
    fn test_non_palindrome() {
        // "hello" != "olleh"
        assert!(!is_palindrome("hello"));
    }
}

fn main() {
    println!("This file demonstrates TDD (Red Phase).");
    println!("Run with `rustc --test` to see the failures!");
}