// Global function (available everywhere)
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Local function (only available inside main)
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    // Call the local function
    greet("Alice");
    greet("Bob");

    // Call the global function
    let sum = add(1, 2);
    println!("The sum is: {}", sum);
}