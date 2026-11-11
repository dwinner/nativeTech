// 15_structs_with_references.rs

// We declare the lifetime parameter <'a> after the struct name.
// This enforces that an instance of ImportantExcerpt cannot outlive
// the reference it holds in 'part'.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    // We create an instance that borrows a slice of 'novel'.
    // The compiler ensures 'first_sentence' is valid only as long as 'novel' is valid.
    let first_sentence = ImportantExcerpt {
        part: &novel[0..15],
    };

    println!("First sentence: {}", first_sentence.part);
}