// 12_tdd_green_phase.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file:
// 1. Compile: rustc --test 12_tdd_green_phase.rs -o test_runner
// 2. Run:     ./test_runner
//
// EXPECTED OUTPUT:
// All tests should PASS (ok). This is the "Green" phase.
// ---------------------------------------------------------

// --- The Implementation (Working) ---

pub fn is_palindrome(s: &str) -> bool {
    // 1. Handle edge case (though the logic below handles it too, being explicit is fine)
    if s.is_empty() {
        return true; 
    }
    
    // 2. Convert to vectors of chars to handle Unicode correctly
    // We create two vectors: one forward, one reversed.
    let forward_chars: Vec<char> = s.chars().collect();
    let reversed_chars: Vec<char> = s.chars().rev().collect();
    
    // 3. Compare them
    forward_chars == reversed_chars
}

// --- The Tests (Same as before) ---

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
    println!("This file demonstrates TDD (Green Phase).");
    println!("Run with `rustc --test` to see the passing tests!");
}