// We use the `derive` attribute to give our struct useful default
// functionality.
// - `Debug`: Allows us to print the struct for debugging purposes using `{:?}`.
// - `PartialEq`: Allows us to compare two `User` instances for equality using `==`.
#[derive(Debug, PartialEq)]
struct User {
    // Note: Unlike tuples, you access struct fields by name, so the order
    // in which you declare or instantiate them doesn't matter.
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Assuming the User struct is defined
fn build_user(email: String, username: String) -> User {
    User {
        email, // Shorthand for email: email
        username, // Shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}

// Chapter 5

fn main() {
    let user_email = String::from("shorthand@example.com");
    let user_name = String::from("shorthand_user");
    let user2 = build_user(user_email, user_name);

    println!("User 2 active status: {}", user2.active);
}