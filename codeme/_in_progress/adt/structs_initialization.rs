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

// Assuming the User struct is defined as above
// struct User { email: String, username: String, active: bool, sign_in_count: u64 }
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // We'll see how to access these fields next
    println!("Created user with email: {}", user1.email);
}