fn greet(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

fn main() {
    let message = greet("Alice", 30);
    println!("{}", message);
}