fn main() {
    let framework = String::from("Actix");

    display_framework(&framework); // Borrow framework immutably
    println!("Framework: {}", framework); // framework is still valid here
}

fn display_framework(s: &String) {
    println!("Using framework: {}", s);
}