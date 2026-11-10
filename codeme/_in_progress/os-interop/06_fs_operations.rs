// 06_fs_operations.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn demonstrate_fs_operations() -> io::Result<()> {
    let playground_dir = PathBuf::from("my_temp_playground");

    // 1. Create a directory structure
    if playground_dir.exists() {
        // Clean up from previous run if necessary (use with caution)
        fs::remove_dir_all(&playground_dir)?;
        println!("Cleaned up existing '{}'", playground_dir.display());
    }

    fs::create_dir_all(&playground_dir)?;
    println!("Created directory: '{}'", playground_dir.display());

    let notes_subdir = playground_dir.join("notes");
    fs::create_dir(&notes_subdir)?;
    println!("Created subdirectory: '{}'", notes_subdir.display());

    // 2. Create and write to a file
    let important_file = notes_subdir.join("important.txt");
    fs::write(&important_file, "Initial notes for the project.")?;
    println!("Created and wrote to: '{}'", important_file.display());

    let draft_file = notes_subdir.join("draft.md");
    fs::write(&draft_file, "# My Draft\n\nThis is a draft document.")?;
    println!("Created and wrote to: '{}'", draft_file.display());

    // 3. List directory contents and get metadata
    println!("\nContents of '{}':", notes_subdir.display());
    for entry_result in fs::read_dir(&notes_subdir)? {
        let entry = entry_result?; // Each entry itself is a Result
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        
        let entry_type = if metadata.is_dir() {
            "DIR"
        } else if metadata.is_file() {
            "FILE"
        } else {
            "OTHER"
        };

        let size = if metadata.is_file() { metadata.len() } else { 0 };

        println!(" - [{}] {} (Size: {} bytes)", 
            entry_type, 
            path.file_name().unwrap_or_default().to_string_lossy(),
            size);
    }

    // 4. Copy and Rename a file
    let copied_file_path = playground_dir.join("important_backup.txt");
    fs::copy(&important_file, &copied_file_path)?;
    println!("\nCopied '{}' to '{}'", important_file.display(), copied_file_path.display());

    let renamed_draft_path = notes_subdir.join("final_ideas.md");
    fs::rename(&draft_file, &renamed_draft_path)?;
    println!("Renamed '{}' to '{}'", draft_file.display(), renamed_draft_path.display());

    // 5. Clean up (remove the entire playground directory)
    // Use with caution in real applications!
    fs::remove_dir_all(&playground_dir)?;
    println!("\nSuccessfully cleaned up and removed directory: '{}'", playground_dir.display());

    Ok(())
}

fn main() {
    if let Err(e) = demonstrate_fs_operations() {
        eprintln!("A file system operation failed: {}", e);
    }
}