struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1 // Copies the remaining fields from user1
    };

    println!("Username: {}", user2.username);
    println!("Email: {}", user2.email);
    println!("Sign in count: {}", user2.sign_in_count);
    println!("Active: {}", user2.active);
}