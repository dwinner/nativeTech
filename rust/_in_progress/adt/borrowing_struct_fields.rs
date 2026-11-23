// Define the necessary User struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // 1. Immutable Borrow:
    // The '&' operator creates an immutable reference to 'user1'.
    // This allows reading the data without taking ownership.
    let user_ref = &user1; 
    
    println!("--- Reading Data via Borrow ---");
    // Accessing the data through the reference (user_ref) is fine.
    println!("Username via reference: {}", user_ref.username); 

    println!("\n--- Original Owner Still Valid ---");
    // 2. The original variable 'user1' is still valid:
    // This works because the data was only borrowed, not moved.
    println!("Email via original variable: {}", user1.email); 

    // If we tried to use the reference to modify the data, it would fail,
    // because it is an immutable borrow (a shared reference).
    // user_ref.active = false; // ERROR: cannot assign to data in an immutable reference
}