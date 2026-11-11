fn main() {
    let x = 5;

    match x {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }
}