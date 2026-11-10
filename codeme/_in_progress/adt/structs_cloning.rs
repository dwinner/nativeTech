// A necessary definition for the User struct
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

    // Create user2 using struct update syntax.
    // - email is newly specified.
    // - username is explicitly cloned, keeping the original data for user1.
    // - ..user1 handles the remaining fields (sign_in_count, active, and email)
    //   Note: The original user1.email field's value is MOVED to user2 to satisfy the ..user1, 
    //         making user1 invalid for future use.
    let user2 = User {
        email: String::from("newemail@example.com"), // New email
        username: user1.username.clone(),            // Explicitly clone String to avoid move
        ..user1                                      // Moves the remaining fields (sign_in_count, active, and the original email value is technically moved but overwritten above)
    };

    println!("--- User 2 Details ---");
    println!("Username: {}", user2.username);
    println!("Email: {}", user2.email);
    println!("Sign-in Count: {}", user2.sign_in_count);
    println!("Active: {}", user2.active);

    // If you try to use the *whole* user1 instance here, it would fail 
    // because the original `email` field was MOVED when using `..user1`.
    // println!("User 1 email: {}", user1.email); // This line would result in a compilation error!
}