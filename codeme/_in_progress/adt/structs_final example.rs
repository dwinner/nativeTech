#[derive(Debug)]
struct UserProfile {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

impl UserProfile {
    // Associated Function (Constructor)
    fn new(username: String, email: String, age: u32) -> UserProfile {
        UserProfile {
            username,
            email,
            age,
            active: true, // Defaulting to active upon creation
        }
    }

    // Mutable Method: Deactivates the user
    fn deactivate(&mut self) {
        self.active = false;
    }

    // Mutable Method: Reactivates the user
    fn reactivate(&mut self) {
        self.active = true;
    }
}

fn main() {
    // The 'mut' keyword is required because we call `deactivate` and `reactivate`
    let mut user = UserProfile::new(
        String::from("johndoe"),
        String::from("johndoe@example.com"),
        30,
    );

    println!("--- Initial Creation ---");
    println!("{:?}", user);

    user.deactivate();
    println!("\n--- After Deactivation ---");
    println!("After deactivation: {:?}", user);

    user.reactivate();
    println!("\n--- After Reactivation ---");
    println!("After reactivation: {:?}", user);
}