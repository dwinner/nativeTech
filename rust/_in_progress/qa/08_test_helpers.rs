// 08_test_helpers.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 08_test_helpers.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

// --- MOCK LIBRARY (Simulating the crate under test) ---
pub mod text_analyzer {
    pub mod analysis {
        pub fn count_words(text: &str) -> usize {
            if text.is_empty() { return 0; }
            text.split_whitespace().count()
        }

        pub fn contains_profanity(text: &str, banned_words: &[&str]) -> bool {
            let lower_text = text.to_lowercase();
            for word in banned_words {
                if lower_text.contains(word) { return true; }
            }
            false
        }
    }
}

// --- HELPER MODULE (Simulating tests/common/mod.rs) ---
// This code is shared across multiple integration tests.
pub mod common {
    pub fn create_sample_long_text() -> String {
        "This is a very long string that we might want to use in multiple integration tests for various analysis purposes. It contains several words and punctuation marks like commas, and even exclamation points!".to_string()
    }

    pub fn common_banned_words_list() -> Vec<&'static str> {
        vec!["heck", "darn", "gosh"]
    }
}

// --- INTEGRATION TESTS ---
#[cfg(test)]
mod tests {
    use super::text_analyzer;
    use super::common; // Import the shared helper module

    #[test]
    fn test_long_text_analysis() {
        // Use the helper to get data
        let text = common::create_sample_long_text();
        
        let count = text_analyzer::analysis::count_words(&text);
        
        // We know our sample text has a specific structure
        assert!(count > 10, "The sample text should be long enough");
    }

    #[test]
    fn test_profanity_with_common_list() {
        // Use the helper to get the standard banned list
        let banned = common::common_banned_words_list();
        
        let clean_input = "This is a clean sentence.";
        let dirty_input = "Oh gosh, I made a mistake.";

        assert!(!text_analyzer::analysis::contains_profanity(clean_input, &banned));
        assert!(text_analyzer::analysis::contains_profanity(dirty_input, &banned));
    }
}

fn main() {
    println!("This file contains integration tests with helper modules.");
    println!("Run with `rustc --test 08_test_helpers.rs`");
}