// 06_library_structure.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run this file normally:
// 1. Compile: rustc 06_library_structure.rs -o text_analyzer
// 2. Run:     ./text_analyzer
// ---------------------------------------------------------

// --- Library Code (Simulating src/lib.rs) ---

// Define a public module 'analysis'
pub mod analysis {
    pub fn count_words(text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }
        text.split_whitespace().count()
    }

    pub fn contains_profanity(text: &str, banned_words: &[&str]) -> bool {
        let lower_text = text.to_lowercase();
        for word in banned_words {
            if lower_text.contains(word) {
                return true;
            }
        }
        false
    }
}

pub struct TextStats {
    pub word_count: usize,
    pub character_count: usize,
}

// A public function exposed at the crate root
pub fn gather_stats(text: &str) -> TextStats {
    TextStats {
        word_count: analysis::count_words(text), // Uses the internal public module
        character_count: text.chars().count(),
    }
}

// --- End of Library Code ---

// In a real project, the code below would be in 'src/main.rs' or 'tests/integration_test.rs'
// Here, we use main to verify the library code works as expected.
fn main() {
    let sample_text = "Rust is amazing";
    
    // 1. Test the top-level function
    let stats = gather_stats(sample_text);
    println!("Stats for '{}':", sample_text);
    println!(" - Words: {}", stats.word_count);         // Expected: 3
    println!(" - Characters: {}", stats.character_count); // Expected: 15

    // 2. Test the inner module function
    let banned = ["bad", "terrible"];
    let clean_text = "Rust is great";
    let dirty_text = "This code is terrible";

    let is_clean_bad = analysis::contains_profanity(clean_text, &banned);
    let is_dirty_bad = analysis::contains_profanity(dirty_text, &banned);

    println!("\nProfanity Check:");
    println!(" - '{}' contains profanity? {}", clean_text, is_clean_bad); // Expected: false
    println!(" - '{}' contains profanity? {}", dirty_text, is_dirty_bad); // Expected: true
}