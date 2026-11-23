use std::fmt;
use std::error::Error; 
use std::num::ParseIntError; 
use std::io; 

// --- Enum defined as before ---
#[derive(Debug)] 
enum DataProcessingError {
    FileNotFound(String),      
    InvalidFormat(ParseIntError), 
    NegativeValue(i32),        
    IoError(io::Error),        
}

// Implement Display for user-friendly output (e.g., with {})
impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataProcessingError::FileNotFound(name) => write!(f, "Could not find file '{}'", name),
            DataProcessingError::InvalidFormat(_) => write!(f, "Invalid data format: Expected an integer"),
            DataProcessingError::NegativeValue(val) => write!(f, "Found negative value {}, expected positive.", val),
            DataProcessingError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

// Implement the Error trait
impl Error for DataProcessingError {
    // The 'source()' method returns the underlying error that caused this error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DataProcessingError::InvalidFormat(e) => Some(e),
            DataProcessingError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

// --- Example Usage ---
fn main() {
    // 1. Example of an IO Error
    let io_err = io::Error::from(io::ErrorKind::NotFound);
    let error1 = DataProcessingError::IoError(io_err);
    
    // FIX: Removed the empty '{}' from the println! call.
    println!("--- 1. Display Output ---"); 
    // Uses the custom fmt::Display implementation
    println!("Error: {}", error1); 

    println!("\n--- 2. Error Source Output ---");
    // The .source() method now works correctly.
    if let Some(source) = error1.source() {
        println!("Underlying Source Type: {:?}", source);
        println!("Source Message (Display): {}", source);
    } else {
        println!("No standard source error found.");
    }

    // 2. Example of a Negative Value Error (No source)
    let error2 = DataProcessingError::NegativeValue(-10);
    
    println!("\n--- 3. Error Source Output (No source) ---");
    if let Some(_) = error2.source() {
        // This branch won't execute
    } else {
        println!("Error: {}", error2);
        println!("No standard source error found.");
    }
}