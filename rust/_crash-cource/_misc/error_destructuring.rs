let failed_parse: Result<i32, _> = "hello".parse();

match failed_parse {
    Ok(number) => println!("Success: {}", number),
    Err(e) => println!("Failed to parse. The error was: {}", e), // Here,
    // 'e' is the destructured error.
}