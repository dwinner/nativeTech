// 17_lifetime_elision.rs

// 1. The Elided Version (What you write)
// The compiler sees one input reference and one output reference.
// It assumes they must be related, so it adds the lifetimes automatically.
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 2. The Explicit Version (What the compiler "expands" it to)
// This is semantically identical to the function above.
// You usually don't need to write this unless you want to be extra specific.
fn get_first_word_explicit<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let text = String::from("Rust is amazing");

    // Both functions work exactly the same way
    let word1 = get_first_word(&text);
    let word2 = get_first_word_explicit(&text);

    println!("Word 1: {}", word1);
    println!("Word 2: {}", word2);
}