//01_basic_read_write.rs
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let sample_path = Path::new("my_sample_file.txt");

    // --- Success Case: Read an existing file ---
    // Corresponds to the flowchart logic: Writing -> Create/Overwrite -> std::fs::write
    // We create the file first so we have something to read.
    if let Err(e) = fs::write(sample_path, "Hello from a Rust test file!") {
        eprintln!("Setup failed: Could not write to sample file: {}", e);
        process::exit(1);
    }

    println!("--- Attempting to read '{}' ---", sample_path.display());

    // Corresponds to the flowchart logic: Reading -> Entire file (small/medium) -> std::fs::read_to_string
    match fs::read_to_string(sample_path) {
        Ok(contents) => {
            println!("Success! File contents: '{}'", contents);
        }
        Err(e) => {
            // This part shouldn't run if the file was created successfully.
            eprintln!("Unexpected error reading the file: {}", e);
        }
    }

    // Clean up the created file so the test is repeatable.
    let _ = fs::remove_file(sample_path);

    println!("\n--------------------------------\n");

    // --- Failure Case: Try to read a non-existent file ---
    let non_existent_path = Path::new("no_such_file.txt");
    println!("--- Attempting to read '{}' ---", non_existent_path.display());

    match fs::read_to_string(non_existent_path) {
        Ok(_) => {
            // This should not happen.
            println!("Unexpectedly found a file that should not exist!");
        }
        Err(e) => {
            // This is the expected outcome.
            eprintln!("Correctly failed to read non-existent file. Error: {}", e);
        }
    }
}