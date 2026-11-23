// 03_read_lines.rs
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let log_file_path = Path::new("events.log");

    // Setup: Create a dummy log file for the example.
    fs::write(log_file_path, "INFO: User logged in.\nWARN: Disk space is low.\nERROR: Failed to fetch resource.")?;

    println!("--- Reading '{}' line by line ---", log_file_path.display());

    // 1. Open the file. The '?' operator propagates errors.
    let file = File::open(log_file_path)?;

    // 2. Wrap the file in a BufReader for efficient, buffered reading.
    let reader = BufReader::new(file);

    // 3. Use the .lines() method to get an iterator over each line.
    for (index, line_result) in reader.lines().enumerate() {
        // Each line is a Result, as I/O can fail mid-read.
        // '?' will propagate the error if a line is malformed or read fails.
        let line = line_result?;

        // Process the line.
        println!("[Line {}] Content: {}", index + 1, line);
    }

    // Cleanup: Remove the dummy file. .ok() ignores a potential error if removal fails.
    fs::remove_file(log_file_path).ok();

    Ok(())
}