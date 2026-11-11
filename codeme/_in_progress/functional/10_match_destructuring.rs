// 10_match_destructuring.rs

enum Message {
    Quit,
    Write(String),
    ChangeColor(u8, u8, u8),
    Move { x: i32, y: i32 },
}

struct User {
    id: u32,
    name: String,
    active: bool,
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting."),
        // Bind the String inside Write to 'text'
        Message::Write(text) => println!("Message to write: {}", text),
        // Bind the RGB values
        Message::ChangeColor(r, g, b) => println!("Change color to R:{} G:{} B:{}", r, g, b),
        // Destructure the struct variant, binding fields x and y
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    }
}

fn describe_user(user: User) {
    // When matching by value (match user), fields must be bound if you want to use them in the arm.
    match user {
        // Match specific ID and active status, bind the name
        User { id: 1, name, active: true } => println!("Admin user '{}' is active.", name),
        
        // Match inactive users. Bind the 'id' field for use in the arm, ignore others with '..'
        User { id, active: false, .. } => println!("User {} is inactive.", id), 

        // Catch-all: Match any other user. Bind the 'id' and 'name'.
        User { id, name, .. } => println!("Regular user #{} is '{}'.", id, name),
    }
}

fn main() {
    println!("--- Message Processing ---");
    process_message(Message::ChangeColor(255, 0, 128));
    process_message(Message::Move { x: 10, y: -5 });

    println!("\n--- User Matching ---");
    let user1 = User { id: 1, name: "Alice".to_string(), active: true };
    let user2 = User { id: 2, name: "Bob".to_string(), active: false };
    let user3 = User { id: 3, name: "Charlie".to_string(), active: true };
    
    describe_user(user1); // Matches: Admin user 'Alice' is active.
    describe_user(user2); // Matches: User 2 is inactive.
    describe_user(user3); // Matches: Regular user #3 is 'Charlie'.
}