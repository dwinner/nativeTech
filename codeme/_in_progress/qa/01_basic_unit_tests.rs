// 01_basic_unit_tests.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 01_basic_unit_tests.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

// The function we want to test.
pub fn capitalize_first_letter(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(), 
        Some(first_char) => {
            first_char.to_uppercase().to_string() + chars.as_str().to_lowercase().as_str()
        }
    }
}

// This module contains the tests.
#[cfg(test)] 
mod tests {
    use super::*;

    #[test] 
    fn test_capitalize_basic_string() {
        let input = "hello";
        let expected = "Hello";
        assert_eq!(capitalize_first_letter(input), expected);
    }

    #[test]
    fn test_capitalize_already_capitalized() {
        assert_eq!(capitalize_first_letter("World"), "World");
    }

    #[test]
    fn test_capitalize_mixed_case() {
        assert_eq!(capitalize_first_letter("rUsT"), "Rust");
    }

    #[test]
    fn test_capitalize_empty_string() {
        assert_eq!(capitalize_first_letter(""), "");
    }

    #[test]
    fn test_capitalize_single_char_string() {
        assert_eq!(capitalize_first_letter("a"), "A");
        assert_eq!(capitalize_first_letter("Z"), "Z");
    }

    #[test]
    fn test_capitalize_with_numbers_and_symbols() {
        assert_eq!(capitalize_first_letter("1st place!"), "1st place!");
        assert_eq!(capitalize_first_letter("!leadingSymbol"), "!leadingsymbol");
    }
}

fn main() {
    println!("This file contains unit tests.");
    println!("Run with `rustc --test 01_basic_unit_tests.rs`");
}