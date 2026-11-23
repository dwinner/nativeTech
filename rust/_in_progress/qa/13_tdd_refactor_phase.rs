// 13_tdd_refactor_phase.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file:
// 1. Compile: rustc --test 13_tdd_refactor_phase.rs -o test_runner
// 2. Run:     ./test_runner
//
// EXPECTED OUTPUT:
// All tests should still PASS.
// If they fail, we broke the code during refactoring.
// ---------------------------------------------------------

// --- The Implementation (Refactored) ---

pub fn is_palindrome(s: &str) -> bool {
    // OLD WAY (Green Phase):
    // let forward_chars: Vec<char> = s.chars().collect();
    // let reversed_chars: Vec<char> = s.chars().rev().collect();
    // forward_chars == reversed_chars
    
    // NEW WAY (Refactored):
    // We use the Iterator trait's `eq` method.
    // 1. `s.chars()` creates an iterator over the string.
    // 2. `s.chars().rev()` creates an iterator moving backwards.
    // 3. `.eq()` compares them item-by-item.
    //
    // Benefits:
    // - No Vec<char> allocation (Zero allocation).
    // - Short-circuiting: It stops as soon as it finds a mismatch.
    // - Much shorter code.
    
    s.chars().eq(s.chars().rev())
}

// --- The Tests (UNCHANGED) ---
// In the Refactor phase, we DO NOT touch the tests.
// They act as our safety net.

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test_empty_string_is_palindrome() {
        assert!(is_palindrome("")); 
    }

    #[test]
    fn test_single_char_is_palindrome() {
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_simple_palindrome() {
        assert!(is_palindrome("madam"));
    }

    #[test]
    fn test_non_palindrome() {
        assert!(!is_palindrome("hello"));
    }
}

fn main() {
    println!("This file demonstrates TDD (Refactor Phase).");
    println!("We improved the code, and the tests prove it still works.");
}