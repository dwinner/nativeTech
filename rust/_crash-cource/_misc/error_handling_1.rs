fn main() {
    // This string can be successfully parsed into a number.
    let number_str = "42";
    // .parse() attempts the conversion. .unwrap() gets the successful
    // value.
    let number = number_str.parse::<i32>().unwrap();
    println!("Successfully parsed number: {}", number);

    // This string CANNOT be parsed into a number.
    let invalid_str = "hello world";

    // The line below would cause the program to panic and crash.
    // We use .expect() to provide a clear message upon failure.

    // let invalid_number = invalid_str.parse::<i32>()
    //     .expect("Failed to parse the string into a number!");

    // Because the line above is commented out, this program will run
    // without error.
    // If you uncomment it, the program will panic and this line will not
    // be reached.
    println!("This line will not be reached if the expect() call
panics.");
}