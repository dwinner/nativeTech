// Define an enum where each variant can optionally carry different kinds of data.
enum Message {
    Quit,
    ChangeColor(u8, u8, u8), // Tuple-like variant
    Move { x: i32, y: i32 }, // Struct-like variant
    Write(String),           // Variant holding a single String
}

// A function to process different message types
fn process_message(msg: Message) {
    // The match expression handles each variant exhaustively and safely destructures the data.
    match msg {
        Message::Quit => {
            println!("Received Quit message: The program should terminate")
        },
        // Destructure the three u8 values into r, g, and b variables
        Message::ChangeColor(r, g, b) => {
            println!("Received ChangeColor: Changing color to RGB({}, {}, {})", r, g, b)
        },
        // Destructure the named fields into x and y variables
        Message::Move { x, y } => {
            println!("Received Move: Moving to coordinates ({}, {}).", x, y)
        },
        // Destructure the String into the 'text' variable
        Message::Write(text) => {
            println!("Received Write: Message content is '{}'.", text)
        },
    }
}

fn main() {
    // Create instances of each variant
    let msg1 = Message::Quit;
    let msg2 = Message::ChangeColor(255, 0, 0); // Red
    let msg3 = Message::Move { x: 100, y: 50 };
    // Note: This String data is MOVED into the enum variant
    let msg4 = Message::Write(String::from("Rust enums are powerful!")); 
    
    // Process each message
    println!("--- Processing Messages ---");
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}