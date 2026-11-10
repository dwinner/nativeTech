// 17_31_unclear_error_messages.rs

fn main() {
    // --- The Pitfall: Generic Mismatches ---
    // Error messages in generic code can be confusing.
    // Here, `T` implements `AsRef<str>`, which means it *can be viewed* as a string slice.
    // However, the inner function demands an owned `String`.
    // Rust won't automatically convert `T` to `String` just because it feels "close enough".
    
    // process_item("hello");

    // --- The Fix: Explicit Conversion ---
    process_item_fixed("hello");
    process_item_fixed(String::from("world"));
}

// Pitfall: The compiler sees `T` (opaque type), but `take_string` wants `String` (concrete type).
/*
fn process_item<T: AsRef<str>>(item: T) {
    // Error: mismatched types
    // expected struct `String`, found type parameter `T`
    take_string(item); 
}
*/

// Fix: Use the trait method to get the reference, then convert to owned String.
fn process_item_fixed<T: AsRef<str>>(item: T) {
    // 1. item.as_ref() turns T -> &str
    // 2. .to_string() turns &str -> String
    let s = item.as_ref().to_string();
    take_string(s);
}

fn take_string(s: String) {
    println!("Took a string: {}", s);
}