use std::fmt;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Manually implementing the Debug trait
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // You have full control over the output format here.
        // We use write! to format the string into the Formatter 'f'.
        write!(
            f,
            "User {{ username: {}, email: {}, sign_in_count: {}, active: {} }}",
            self.username, self.email, self.sign_in_count, self.active
        )
    }
}

fn main() {
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("--- Output using the Custom Debug Implementation ({{:?}}) ---");
    // This calls the fmt function we defined above.
    println!("{:?}", user1); 
    println!("\nThis output exactly matches the string defined in the write! macro.");
}