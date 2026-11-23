// handling_standard_input.rs
// Demonstrates the use of println! for standard output (stdout)
// and eprintln! for standard error (stderr).

use std::io::{self, BufRead};

// Note: In a real-world application, file_path would be a variable.
const DUMMY_FILE_PATH: &str = "/etc/config/settings.ini";

fn main() {
    // --- 1. Standard Output (stdout) ---
    // Use println! for the program's intended result or main information flow.
    println!("Application started successfully.");
    println!("Please enter text line by line. Type 'exit' to finish.");

    // --- 2. Standard Error (stderr) ---
    // Use eprintln! for diagnostics, warnings, and errors.
    // This output bypasses standard output pipes, making it ideal for logging issues.

    // Example of a warning:
    eprintln!("Warning: Using default processing settings.");
    
    // Example of a diagnostic message during an operation:
    eprintln!("Diagnostic: Initializing I/O buffer.");

    // Simulate reading from standard input
    let stdin = io::stdin();
    let mut line_count = 0;
    
    // Read user input line by line
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim().eq_ignore_ascii_case("exit") {
                    eprintln!("Diagnostic: Exiting input loop.");
                    break;
                }
                line_count += 1;
                // Outputting the processed result to stdout
                println!("Echo (Line {}): {}", line_count, input);
            }
            Err(error) => {
                // Outputting a critical error to stderr
                eprintln!("Error: Failed to read line: {}", error);
                break;
            }
        }
    }

    // Example of a final status message to stdout
    println!("Processing complete. Processed {} lines.", line_count);
    
    // Example of an error that might occur during cleanup
    eprintln!("Error: Input file not found at path '{}'", DUMMY_FILE_PATH);
}