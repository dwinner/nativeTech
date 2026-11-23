// 17_11_ignoring_result.rs
use std::fs::File;
use std::io::Write;

fn main() {
    // --- The Pitfall: Ignoring Result Types ---
    // File::create returns a Result. If we ignore it, we miss potential failures 
    // (like permission denied), and the compiler will issue a warning:
    // "unused `Result` that must be used"
    
    File::create("my_output_file.txt"); 
    // The program continues here even if file creation failed!
    
    println!("Pitfall executed: Assumed file was created, but never checked.");

    // --- The Fix: Handling the Result ---
    // We explicitly check if the operation succeeded (Ok) or failed (Err).

    match File::create("my_output_file.txt") {
        Ok(mut file) => {
            println!("Fix executed: File created successfully!");
            
            // Even writing to the file returns a Result that should be checked
            if let Err(e) = file.write_all(b"Hello, Rust!") {
                eprintln!("Error writing to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            // We can now handle the error gracefully (retry, exit, log, etc.)
        }
    }
}