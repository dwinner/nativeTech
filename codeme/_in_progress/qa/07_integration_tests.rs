// 07_integration_tests.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 07_integration_tests.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

// --- MOCK LIBRARY (Simulating an external crate named "text_analyzer") ---
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

    pub struct TextStats {
        pub word_count: usize,
        pub character_count: usize,
    }

    pub fn gather_stats(text: &str) -> TextStats {
        TextStats {
            word_count: analysis::count_words(text),
            character_count: text.chars().count(),
        }
    }
}
// --- END MOCK LIBRARY ---


// --- INTEGRATION TESTS (Simulating tests/analyzer_integration.rs) ---
#[cfg(test)]
mod tests {
    // We import the "crate" (our module) just like an external user would.
    // In a real integration test, this would be `use text_analyzer;`
    use super::text_analyzer; 

    #[test]
    fn test_word_count_integration() {
        let sample_text = "This is a sample sentence.";
        
        // Call the public function from our library's public module
        let count = text_analyzer::analysis::count_words(sample_text);
        
        assert_eq!(count, 5, "Word count should be 5");
    }

    #[test]
    fn test_profanity_checker_integration() {
        let sample_text_clean = "A lovely day for a walk.";
        let sample_text_profane = "This is a darn naughty sentence.";
        let banned = ["darn", "naughty"];
        
        assert!(
            !text_analyzer::analysis::contains_profanity(sample_text_clean, &banned), 
            "Clean text should not contain profanity"
        );
        
        assert!(
            text_analyzer::analysis::contains_profanity(sample_text_profane, &banned), 
            "Profane text should be detected"
        );
    }

    #[test]
    fn test_gather_stats_integration() {
        let sample_text = "Hello world!"; // 2 words, 12 chars
        let stats = text_analyzer::gather_stats(sample_text);
        
        // We can access public fields of structs returned by public functions
        assert_eq!(stats.word_count, 2);
        assert_eq!(stats.character_count, 12);
    }

    #[test]
    fn test_empty_string_stats() {
        let stats = text_analyzer::gather_stats("");
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.character_count, 0);
    }
}

fn main() {
    println!("This file contains integration tests.");
    println!("Run with `rustc --test 07_integration_tests.rs`");
}