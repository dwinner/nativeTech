fn main() {
    let book = String::from("Rust Programming");

    let len = calculate_length(&book); // Borrow book immutably
    println!("The length of '{}' is {}.", book, len);

    // book is still valid here
}

fn calculate_length(s: &String) -> usize {
    s.len()
}