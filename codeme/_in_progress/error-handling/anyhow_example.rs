use anyhow::{Context, Result, bail}; // Import Result, Context, bail
use std::fs;
use std::path::Path; // Use Path for clarity

// --- DEPENDENCY NOTE ---
// To use this code, you must add the 'anyhow' dependency to your Cargo.toml:
// [dependencies]
// anyhow = "1.0"
// -----------------------

// The function now returns anyhow::Result<i32> which is a type alias for Result<i32, anyhow::Error>.
fn read_positive_number(path: &Path) -> Result<i32> {
    // 1. Read file content: returns Result<String, io::Error>
    // The '?' operator propagates the io::Error, and .context() adds specific, useful text.
    let content = fs::read_to_string(path)
        .context(format!("Failed to read file '{}'", path.display()))?;

    // 2. Parse number: returns Result<i32, ParseIntError>
    let number = content.trim().parse::<i32>()
        .context("File content is not a valid integer")?;

    // 3. Validate positivity (Domain Error)
    if number < 0 {
        // bail! is an easy way to create and return an anyhow::Error
        bail!("The number read ({}) must be positive.", number);
    }
    
    Ok(number)
}

// main can also return anyhow::Result<()> for easy error propagation
fn main() -> Result<()> {
    let valid_path = Path::new("valid_data_ah.txt");
    fs::write(valid_path, "789").unwrap();
    
    // --- Test 1: Success ---
    let number = read_positive_number(valid_path)?;
    println!("Success with anyhow: {}", number);
    fs::remove_file(valid_path).ok();
    
    // --- Test 2: Error (Negative Value) ---
    let negative_path = Path::new("negative_data_ah.txt");
    fs::write(negative_path, "-5").unwrap();
    println!("\n--- Test 2: Negative Value Error ---");
    match read_positive_number(negative_path) {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => {
            // anyhow formats the error including context and cause chain
            // Output (approx): Error: The number read (-5) must be positive.
            eprintln!("Error with anyhow: {:?}", e);
        }
    }
    fs::remove_file(negative_path).ok();
    
    // --- Test 3: Error (File not found - Context is shown) ---
    println!("\n--- Test 3: File Not Found Error ---");
    let nonexistent_path = Path::new("nonexistent_ah.txt");
    match read_positive_number(nonexistent_path) {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => {
            // Output (approx): Error: Failed to read file 'nonexistent_ah.txt'
            eprintln!("Error with anyhow: {:?}", e);
        }
    }

    Ok(())
}