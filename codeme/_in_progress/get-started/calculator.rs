use std::io;

fn main() {
    // Display a welcome message to introduce the program
    println!("Welcome to the Rust CLI Calculator!");

    // Prompt the user to enter the first number
    // We use a custom function `get_number` to handle input validation
    let num1 = get_number("Enter the first number: ");

    // Prompt the user to enter the second number
    let num2 = get_number("Enter the second number: ");

    // Prompt the user to enter an operation
    println!("Enter an operation (+, -, *, /):");

    // Create a mutable string to store the user's input
    let mut operation = String::new();

    // Read the user's input and store it in `operation`
    io::stdin().read_line(&mut operation).expect("Failed to read input");

    // Trim any whitespace (e.g., newline) from the input
    let operation = operation.trim();

    // Perform the calculation based on the chosen operation
    let result = match operation {
        "+" => Some(num1 + num2), // Addition
        "-" => Some(num1 - num2), // Subtraction
        "*" => Some(num1 * num2), // Multiplication
        "/" => {
            // Before dividing, check that the second number is not zero
            if num2 != 0.0 {
                Some(num1 / num2) // Division
            } else {
                println!("Error: Division by zero is not allowed.");
                None // Return None if division by zero is attempted
            }
        }
        _ => {
            // If the user enters an invalid operation, print an error
            // message
            println!("Invalid operation. Please enter +, -, *, or /.");
            None // Return None to indicate an invalid operation
        }
    };

    // If the result is valid (not None), print the result
    if let Some(res) = result {
        println!("Result: {}", res);
    }
}

// This function prompts the user to enter a number and ensures valid input
fn get_number(prompt: &str) -> f64 {
    loop { // Continue looping until valid input is provided
        println!("{}", prompt); // Display the prompt message

        let mut input = String::new(); // Create a new mutable string for user input
        io::stdin().read_line(&mut input).expect("Failed to read input"); // Read input

        // Try to convert the input string into a floating-point number (f64)
        match input.trim().parse::<f64>() {
            Ok(num) => return num, // If parsing succeeds, return the number
            Err(_) => println!("Invalid number. Please enter a valid numeric value."), // If parsing fails, prompt again
        }
    }
}