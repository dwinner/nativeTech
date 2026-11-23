// 10_multiple_trait_bounds.rs

use std::fmt::Debug;

// This function accepts any type `T` that implements BOTH `Debug` AND `Clone`.
// The '+' syntax represents an intersection of requirements.
fn process_item<T: Debug + Clone>(item: T) {
    // We can call .clone() because T implements Clone
    let item_clone = item.clone();
    
    // We can use "{:?}" because T implements Debug
    println!("Processing item: {:?}", item);
    println!("And its clone: {:?}", item_clone);
}

// A custom struct to demonstrate deriving the necessary traits
#[derive(Debug, Clone)]
struct UserID {
    id: u32,
    group: String,
}

fn main() {
    // 1. Works with primitives (i32 implements Debug and Clone)
    let number = 42;
    process_item(number);

    println!("---");

    // 2. Works with standard library types (String implements Debug and Clone)
    let text = String::from("Hello Rust");
    process_item(text);

    println!("---");

    // 3. Works with custom structs (if they derive Debug and Clone)
    let user = UserID {
        id: 101,
        group: String::from("Admins"),
    };
    process_item(user);
}