// 07_read_file_lines.rs
//
// To test this code, you can use the following commands in your terminal:
// 1. Built-in test: `cargo run -- sample_cli_read.txt`
// 2. Read specific file: `cargo run -- my_notes.txt`
// 3. Test missing file: `cargo run -- non_existent_file.txt`
// 4. Usage help: `cargo run`

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process; // For process::exit

// This function attempts to read and print lines from a file.
// It's similar to what our mini_grep will need to do before searching.
fn read_and_print_file_lines(file_path_str: &str) {
    let path = Path::new(file_path_str);

    // Attempt to open the file
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open file '{}': {}", path.display(), e);
            process::exit(1); // Exit with error code
        }
    };

    // Use BufReader for efficient line-by-line reading
    let reader = BufReader::new(file);

    println!("--- Contents of '{}' ---", path.display());

    for (index, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(line_content) => {
                println!("Line {}: {}", index + 1, line_content);
            }
            Err(e) => {
                // Log error for a specific line but continue if possible,
                // or decide to exit if line read errors are critical.
                eprintln!("Error reading line {} from '{}': {}", index + 1, path.display(), e);
                // For a grep tool, we might want to skip unreadable lines
                // or halt. For now, we'll just report and continue.
            }
        }
    }

    println!("--- End of '{}' ---", path.display());
}

fn main() {
    // Simulate getting a file path from command-line arguments
    // In a real CLI, this would come from std::env::args() or a parsing crate.
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args.get(0).unwrap_or(&"program_name".into()));
        process::exit(1);
    }

    let file_to_read = &args[1];

    // Create a dummy file for testing if it doesn't exist
    if !Path::new(file_to_read).exists() {
        if file_to_read == "sample_cli_read.txt" { // Only create if it's our expected test file
            std::fs::write(file_to_read, "First line for CLI test.\nSecond line, with a keyword.\nThird and final line.").expect("Failed to create sample file.");
            println!("Created sample file: {}", file_to_read);
        } else {
            eprintln!("Specified file '{}' does not exist and won't be auto-created for this generic example.", file_to_read);
            process::exit(1);
        }
    }

    read_and_print_file_lines(file_to_read);

    // Clean up the dummy file if we created it for the test
    if file_to_read == "sample_cli_read.txt" {
        std::fs::remove_file(file_to_read).ok();
    }
}