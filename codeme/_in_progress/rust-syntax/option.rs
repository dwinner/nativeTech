fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    match some_number {
        Some(x) => println!("The number is: {}", x),
        None => println!("No number"),
    }

    match absent_number {
        Some(x) => println!("The number is: {}", x),
        None => println!("No number"),
    }
}