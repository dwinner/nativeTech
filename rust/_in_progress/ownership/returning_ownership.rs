fn main() {
    let my_string = String::from("Hello, Rust!");

    let my_string = return_ownership(my_string);

    println!("{}", my_string); // Now this is valid
}

fn return_ownership(s: String) -> String {
    s
}