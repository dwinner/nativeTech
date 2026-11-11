// 17_37_ownership_model.rs
fn process_by_ownership(s: String) {
    println!("Took ownership: {}", s);
    // 's' is dropped here, memory freed
}

fn process_by_borrow(s: &str) {
    println!("Borrowed: {}", s);
    // 's' is just a reference, underlying data stays alive
}

fn main() {
    let my_string = String::from("Hello, Rust!");

    // 1. Clone: Duplicates data. Use when the function needs ownership
    // but you still need the original.
    process_by_ownership(my_string.clone());
    println!("Alive after clone: {}", my_string);

    // 2. Borrow: Most efficient. Pass a reference for read-only access.
    process_by_borrow(&my_string);
    println!("Alive after borrow: {}", my_string);

    // 3. Move: Transfers ownership. Efficient, but invalidates 'my_string'.
    process_by_ownership(my_string);
    // println!("{}", my_string); // Compile Error: value used after move
}