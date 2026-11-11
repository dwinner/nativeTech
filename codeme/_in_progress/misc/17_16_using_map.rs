// 17_16_using_map.rs

fn main() {
    // --- 1. Using map with Option ---
    // The map combinator applies a function to the value inside Some, 
    // wrapping the result in a new Option. If it's None, it returns None.

    let maybe_number_str: Option<&str> = Some("5");
    
    // Transform Some("5") -> Some(5)
    // We use unwrap_or(-1) to handle the parsing result internally, 
    // ensuring we return an i32 to the map function.
    let maybe_number_int: Option<i32> = maybe_number_str.map(|s| {
        s.parse::<i32>().unwrap_or(-1)
    });

    println!("Mapped Option (Some): {:?}", maybe_number_int); // Some(5)

    let no_number_str: Option<&str> = None;
    // The closure is never executed because the Option is None.
    let no_number_int: Option<i32> = no_number_str.map(|s| {
        s.parse::<i32>().unwrap_or(-1)
    });

    println!("Mapped Option (None): {:?}", no_number_int); // None

    // --- 2. Using map with Result ---
    // Similarly, map works on the Ok variant, leaving Err untouched.

    let good_result: Result<&str, &str> = Ok("10");
    // Transform Ok("10") -> Ok(2) (the length)
    let mapped_result: Result<usize, &str> = good_result.map(|s| s.len());

    println!("Mapped Result (Ok): {:?}", mapped_result); // Ok(2)

    let bad_result: Result<&str, &str> = Err("Error occurred");
    // The closure is skipped; the error is propagated as is.
    let mapped_bad_result: Result<usize, &str> = bad_result.map(|s| s.len());

    println!("Mapped Result (Err): {:?}", mapped_bad_result); // Err("Error occurred")
}