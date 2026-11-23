fn main() {
    let maybe_number: Option<i32> = Some(5);
    // If maybe_number is Some(5), `number` becomes 5.
    // If it were None, `number` would become 0.
    let number = maybe_number.unwrap_or(0);
    println!("Using unwrap_or: The number is: {}", number);

    let nothing: Option<i32> = None;
    // Since `nothing` is None, `default_number` becomes 10.
    let default_number = nothing.unwrap_or(10);
    println!("Using unwrap_or: The default number is: {}", default_number);
}