// A basic definition for the User struct to make the code compile
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64, // Using u64 for the count
}

// Assuming the user1 instance from earlier
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Accessing fields using dot notation
    println!("The username is: {}", user1.username);
    println!("Is the user active? {}", user1.active);
}