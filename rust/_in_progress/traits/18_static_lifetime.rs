// 18_static_lifetime.rs

// Example 1: Global constants always have the 'static lifetime.
// They exist in a fixed memory location for the entire program run.
static GREETING: &str = "Welcome to the Static Void";

fn main() {
    // Example 2: String Literals
    // This string data is baked directly into the executable binary.
    // It is valid from the moment the program starts until it exits.
    let s: &'static str = "I have a static lifetime.";

    println!("{}", s);
    println!("{}", GREETING);

    // We can pass these to functions that require data to live forever
    require_static(s);
}

fn require_static(text: &'static str) {
    println!("This text will never die: '{}'", text);
}