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

    // The assignment moves the String data from user1 to user2.
    // user1 is no longer valid.
    let user2 = user1; 

    println!("--- Accessing user2 is fine ---");
    println!("Username: {}", user2.username); 
    
    // --- ERROR Demonstration (Uncommenting this line causes the error!) ---
    // println!("Username: {}", user1.username); 
    // ^^^ This line would fail with "error[E0382]: use of moved value: `user1`"
}