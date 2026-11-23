// 08_list_dir_contents.rs
//
// To test this code:
// 1. List current directory (default): `cargo run`
// 2. List a specific directory (e.g., src): `cargo run -- src`
// 3. Test invalid directory: `cargo run -- invalid_folder`

use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::io;

fn list_directory_contents(dir_path_str: &str) -> io::Result<()> {
    let path = Path::new(dir_path_str);

    if !path.is_dir() {
        // Using eprintln! for error messages is good practice in CLI tools
        eprintln!("Error: '{}' is not a directory or does not exist.", path.display());
        // Return an error that can be handled by the caller if needed
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path is not a directory"));
    }

    println!("Contents of directory '{}':", path.display());

    // fs::read_dir returns a Result containing an iterator over DirEntry results
    for entry_result in fs::read_dir(path)? { // '?' propagates I/O errors from read_dir itself
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                // Error accessing a specific entry, log it and continue
                eprintln!("Warning: Could not access an entry in '{}': {}", path.display(), e);
                continue;
            }
        };

        let entry_path = entry.path();
        let entry_name = entry_path.file_name().unwrap_or_default().to_string_lossy(); // Get just the name part

        // Get metadata to determine if it's a file or directory
        // This can also fail (e.g., permissions)
        match fs::metadata(&entry_path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    println!(" [DIR]   {}", entry_name);
                } else if metadata.is_file() {
                    println!(" [FILE]  {} ({} bytes)", entry_name, metadata.len());
                } else {
                    println!(" [OTHER] {}", entry_name); // Symlinks, etc.
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not get metadata for '{}': {}", entry_path.display(), e);
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let dir_to_list = if args.len() > 1 {
        args[1].clone() // Use the provided argument
    } else {
        // Default to the current directory if no argument is given
        String::from(".")
    };

    println!("Attempting to list contents of '{}'...", dir_to_list);

    if let Err(e) = list_directory_contents(&dir_to_list) {
        // Error was already printed in list_directory_contents if it was path not being a dir.
        // This catches other potential errors from the function signature.
        if e.kind() != io::ErrorKind::NotFound { // Avoid double printing for "not a directory"
            eprintln!("An error occurred: {}", e);
        }
        process::exit(1);
    }
}