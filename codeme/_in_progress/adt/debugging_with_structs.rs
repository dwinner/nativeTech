// The #[derive(Debug)] attribute automatically implements the Debug trait, 
// allowing the struct to be printed using the debug format specifier {:?}.
#[derive(Debug)]
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

    println!("--- Using the Debug Format ({{:?}}) ---");
    println!("{:?}", user1);
    
    // You can also use the pretty-print format {:#?} for multi-line, 
    // indented output, which is often easier to read for complex structs.
    println!("\n--- Using the Pretty-Print Debug Format ({{:#?}}) ---");
    println!("{:#?}", user1);
}