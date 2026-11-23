// 05_append_to_file.rs
use std::fs::{self, OpenOptions};
use std::io::{self, Write, ErrorKind};
use std::path::Path;

/// Opens a file in append mode and writes a new line to it.
/// Creates the file if it doesn't exist.
fn append_line_to_file(file_path: &Path, line_to_append: &str) -> io::Result<()> {
    // Use OpenOptions to configure how the file is opened.
    let mut file = OpenOptions::new()
        .append(true) // Set to append mode.
        .create(true) // Create the file if it does not exist.
        .open(file_path)?;

    // writeln! is convenient for writing a string followed by a newline.
    writeln!(file, "{}", line_to_append)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let log_path = Path::new("application.log");

    // Start with a clean slate for the example.
    // .ok() converts Result to Option, so we ignore errors if file doesn't exist.
    match std::fs::remove_file(log_path) {
        Ok(_) => {
            // Successfully removed the old file, we can optionally log this
            println!("Note: Removed old log file for a clean run.");
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // This is perfectly fine, the file just didn't exist.
            // We can do nothing and continue.
        }
        Err(e) => {
            // This is an unexpected error (like permission denied).
            // We'll print a warning but continue the program.
            eprintln!(
                "Warning: Could not remove old log file '{}': {}. Proceeding anyway.",
                log_path.display(), e
            );
        }
    }

    println!("Preparing to write to '{}'...", log_path.display());

    // Append several lines to the same file.
    append_line_to_file(log_path, "[INFO] Application started.")?;
    append_line_to_file(log_path, "[WARN] Low disk space detected.")?;
    append_line_to_file(log_path, "[INFO] User 'admin' logged in.")?;

    println!("Finished writing log entries.");

    // Verify the final contents of the file.
    let final_content = fs::read_to_string(log_path)?;
    println!("\n--- Final Contents of '{}' ---", log_path.display());
    println!("{}", final_content);
    println!("--------------------------------");

    // Clean up the created file.
    fs::remove_file(log_path)?;

    Ok(())
}