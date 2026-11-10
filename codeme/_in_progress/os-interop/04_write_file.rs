// 04_write_file.rs
use std::fs; // For fs::read_to_string and fs::remove_file
use std::fs::File; // For File::create
use std::io::{self, Write}; // For the Write trait and its methods like write_all
use std::path::Path;

fn save_report_to_file(report_path: &Path, report_content: &str) -> io::Result<()> {
    // Method 1: Using fs::write (convenient for simple, complete writes)
    // This will create the file if it doesn't exist, or truncate and overwrite it if it does.
    // fs::write(report_path, report_content)?;

    // Method 2: Using File::create and write_all for more explicit control
    
    // File::create opens a file in write-only mode.
    // If the file already exists, its content is truncated (emptied).
    // If it does not exist, a new file is created.
    let mut output_file = File::create(report_path)?;

    // The write_all method takes a byte slice (&[u8]).
    // It will attempt to write the entire buffer to the file.
    output_file.write_all(report_content.as_bytes())?;

    println!("Successfully wrote report to '{}'", report_path.display());
    Ok(())
}

fn main() {
    let my_report_path = Path::new("financial_report.txt");
    let report_data = "Q1 Report:\nSales: $1,000,000\nExpenses: $400,000\nProfit: $600,000\n";

    if let Err(e) = save_report_to_file(my_report_path, report_data) {
        eprintln!("Error writing report to file: {}", e);
    } else {
        // Verify by reading it back
        match fs::read_to_string(my_report_path) {
            Ok(content_read) => {
                println!("\n--- Report Read Back for Verification ---");
                println!("{}", content_read);
                println!("--- End of Verification ---");
            }
            Err(e) => eprintln!("Error reading back report for verification: {}", e),
        }
    }

    // Clean up the dummy file
    if let Err(e) = fs::remove_file(my_report_path) {
        eprintln!("Cleanup error: Failed to remove report file '{}': {}", my_report_path.display(), e);
    }
}