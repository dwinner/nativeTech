// --- DEPENDENCY NOTE ---
// To use this code, you must add the following dependencies to your Cargo.toml:
// [dependencies]
// anyhow = "1.0"
// thiserror = "1.0"
// log = "0.4"
// env_logger = "0.9"
// -----------------------

use std::fs::{File, remove_file};
use std::io::{self, BufRead, BufReader, Write};
use thiserror::Error;
use anyhow::{Context, Result};
use log::{info, warn, error};
use env_logger::Env;

// Specific error for this processing logic
#[derive(Error, Debug)]
enum ProcessingError {
    #[error("Invalid number format on line {line_num}: '{content}'")]
    InvalidFormat {
        line_num: usize,
        content: String,
        #[source] // Original source error
        source: std::num::ParseIntError,
    },

    #[error("Negative number {number} not allowed on line {line_num}")]
    NegativeNumber {
        line_num: usize,
        number: i32,
    },
}

// Function to process a single line
fn process_line(line_content: &str, line_num: usize) -> Result<i32, ProcessingError> {
    // Attempt to parse the number
    let number = line_content.trim().parse::<i32>()
        // Map the standard ParseIntError into our custom InvalidFormat error
        .map_err(|e| ProcessingError::InvalidFormat {
            line_num,
            content: line_content.to_string(),
            source: e, // Include original parse error
        })?;

    // Check our domain logic (no negative numbers)
    if number < 0 {
        Err(ProcessingError::NegativeNumber { line_num, number })
    } else {
        Ok(number)
    }
}

// Main file processing function. Returns an anyhow::Result.
fn process_file_and_sum(filename: &str) -> anyhow::Result<i32> {
    info!("Starting file processing: {}", filename);

    // Use anyhow::Context to add info to I/O errors
    let file = File::open(filename)
        .context(format!("Failed to open file '{}'", filename))?;

    let reader = BufReader::new(file);
    let mut total_sum = 0;
    let mut lines_processed = 0;

    for (index, line_result) in reader.lines().enumerate() {
        let line_num = index + 1; // 1-based indexing for user messages
        
        // Use anyhow::Context for I/O errors during line reading
        let line = line_result
            .context(format!("Failed reading line {}", line_num))?;

        // Try to process the line
        match process_line(&line, line_num) {
            Ok(number) => {
                total_sum += number;
                lines_processed += 1;
            }
            Err(e) => {
                // Log the specific ProcessingError, but continue the loop
                warn!("Error processing line {}: {} - Skipping.", line_num, e);
            }
        }
    }

    info!("File processing complete. Lines processed: {}. Total sum: {}", lines_processed, total_sum);
    Ok(total_sum)
}

fn main() -> Result<()> {
    // Initialize logger. Default to "info" if RUST_LOG is not set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Create a test file
    let test_file_name = "test_data.txt";
    let mut file = File::create(test_file_name)?;
    writeln!(file, "10")?;
    writeln!(file, "25")?;
    writeln!(file, "-5")?;   // Error: negative
    writeln!(file, "abc")?;  // Error: format
    writeln!(file, "15")?;
    drop(file); // Ensure file is closed before reading

    // Run the processing function
    match process_file_and_sum(test_file_name) {
        Ok(sum) => {
            info!("Final sum calculated: {}", sum); // Should be 10 + 25 + 15 = 50
            assert_eq!(sum, 50); // Check the result
            println!("Sum calculated successfully: {}", sum);
        }
        Err(e) => {
            // This will catch high-level errors like "File not found"
            error!("Critical error during file processing: {:?}", e);
        }
    }

    std::fs::remove_file(test_file_name)?; // Clean up test file
    Ok(())
}