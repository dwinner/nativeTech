#[derive(Debug)]
enum Color {
    // A variant that holds a tuple of three 8-bit unsigned integers
    Rgb(u8, u8, u8),
    // A variant that holds a single String
    Named(String),
}

fn main() {
    let red = Color::Rgb(255, 0, 0);
    let custom_color = Color::Named(String::from("Forest Green"));

    println!("An RGB color: {:?}", red);
    println!("A named color: {:?}", custom_color);
}