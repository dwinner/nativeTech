#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("original_user"),
        email: String::from("original@example.com"),
        sign_in_count: 50,
        
// Composite Types in Rust

        active: true,
    };
    
    // Create user2 based on user1, but with a new email.
    let user2 = User {
        email: String::from("new_user@example.com"),
        ..user1 // Handle remaining fields
    };

    // The line below would now fail to compile because `user1.username` 
    // was `moved` to `user2`. The rest of `user1`'s fields (`sign_in_
    // count` and `active`) 
    // were copied because they implement the `Copy` trait, but since part
    // of `user1` 
    // was moved, the whole instance cannot be used like this anymore.
    // println!("Original User 1 after move: {:?}", user1);

    println!("New User 2 Details:");
    println!("- username: {} (Moved from user1)", user2.username);
    println!("- email: {} (Newly specified)", user2.email);
    println!("- sign_in_count: {} (Copied from user1, as u64 is Copy)", user2.sign_in_count);
    println!("- active: {} (Copied from user1, as bool is Copy)", user2.active);
}