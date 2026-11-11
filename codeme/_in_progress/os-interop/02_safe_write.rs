// 02_safe_write.rs
use std::fs;
use std::path::Path;

fn main() {
    let filename = "important_data.txt";
    let path = Path::new(filename);

    // 1. First write: Should succeed because file doesn't exist yet
    println!("--- Attempt 1: Writing to new file ---");
    safe_write(path, "This is the original content.");

    println!("\n--------------------------------\n");

    // 2. Second write: Should abort because file exists
    println!("--- Attempt 2: Trying to overwrite ---");
    safe_write(path, "This content should NOT be written.");

    // Cleanup: Remove the file so the test can be run again
    // let _ = fs::remove_file(path); 
}

// Logic extracted and cleaned from your text
fn safe_write(path: &Path, content: &str) {
    if path.exists() {
        // The file already exists. Decide what to do:
        // 1. Return an error.
        // 2. Ask the user for confirmation.
        // 3. Do nothing.
        eprintln!("Error: File '{}' already exists. Aborting to prevent overwrite.", path.display());
    } else {
        // The file doesn't exist, so it's safe to write.
        if let Err(e) = fs::write(path, content) {
            eprintln!("Error writing to new file: {}", e);
        } else {
            println!("Successfully wrote to new file '{}'", path.display());
        }
    }
}