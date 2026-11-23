// This function searches for a word in a sentence.
// It returns Option<usize>: Some(index) if the word is found, or None 
// otherwise.
fn find_word_index(haystack: &str, needle: &str) -> Option<usize> {
    // The built-in .find() method on string slices already returns
    // Option<usize>,
    // which is very convenient.
    haystack.find(needle)
}

fn main() {
    let famous_phrase = "The quick brown fox jumps over the lazy dog.";
    let word_to_find = "fox";
    let word_not_present = "cat";

    // --- Test Case 1: Word is found ---
    println!("Searching for '{}'...", word_to_find);
    match find_word_index(famous_phrase, word_to_find) {

        Some(index) => println!("Success! Found at index {}.", index),
        None => println!("Failure: Word not found."),
    }

    println!("---"); // This line separates the test cases visually

    // --- Test Case 2: Word is NOT found ---
    println!("Searching for '{}'...", word_not_present);
    match find_word_index(famous_phrase, word_not_present) {
        Some(index) => println!("Success! Found at index {}.", index),
        None => println!("Correctly determined that the word was not found."),
    }
}