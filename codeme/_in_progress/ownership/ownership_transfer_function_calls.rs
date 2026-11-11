fn main() {
    let my_string = String::from("Hello, world!");

    take_ownership(my_string);

    // This line would cause a compile-time error
    // println!("{}", my_string);
}

fn take_ownership(s: String) {
    println!("Taking ownership: {}", s);
}