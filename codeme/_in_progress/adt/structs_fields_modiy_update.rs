// 1. Define the Struct
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64, // Use u64 for the count
}

fn main() {
    // 2. Instantiate the struct with 'mut'
    // The 'mut' keyword is essential to allow any field to be changed later.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("--- Before Modification ---");
    println!("Original Email: {}", user1.email);
    println!("Original Sign-In Count: {}", user1.sign_in_count);
    
    // 3. Modify Fields using Dot Notation
    // We can now assign a new value to individual fields because user1 is mutable.
    user1.email = String::from("new.email@example.com");
    user1.sign_in_count += 1; // Incrementing the count

    println!("\n--- After Modification ---");
    println!("Updated Email: {}", user1.email);
    println!("Updated Sign-In Count: {}", user1.sign_in_count);
}