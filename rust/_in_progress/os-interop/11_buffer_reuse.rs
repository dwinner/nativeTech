// 11_buffer_reuse.rs

use std::fs::{self, File};
use std::io::{self, Read, BufReader};
use std::path::Path;

// Define a standard chunk size for reading. 8KB is a common, efficient size.
const CHUNK_SIZE: usize = 8 * 1024; // 8192 bytes

/// Reads a file in chunks, reusing a buffer to count zero bytes.
fn count_zero_bytes(file_path: &Path) -> io::Result<u64> {
    let file = File::open(file_path)?;
    // Wrap in a BufReader for efficiency, even though we read in chunks.
    let mut reader = BufReader::new(file);

    // 1. Allocate the buffer *once*, outside the loop.
    // We use a Vec, which is heap-allocated, as a very large
    // stack-allocated array could cause a stack overflow.
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_zero_bytes = 0;

    loop {
        // 2. Pass a mutable reference to the *existing* buffer to `read`.
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break, // Ok(0) means End of File (EOF). We're done.
            Ok(n) => n,     // `n` is the number of bytes actually read.
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue, // Interrupted by a signal, retry.
            Err(e) => return Err(e), // A real I/O error occurred.
        };

        // 3. Process only the valid part of the buffer (`&buffer[..bytes_read]`).
        // Our "processing" is just counting zero bytes.
        let count_in_chunk = buffer[..bytes_read]
            .iter()
            .filter(|&&b| b == 0x00)
            .count() as u64;

        total_zero_bytes += count_in_chunk;
    }

    Ok(total_zero_bytes)
}

fn main() {
    let test_file = Path::new("chunk_test.dat");

    // --- Setup: Create a dummy file with some zero bytes ---
    let mut content = vec![1, 2, 3, 4, 5, 0, 6, 7, 0, 8];
    content.resize(10_000, 1); // Make it ~10KB with mostly 1s
    content[2000] = 0; // Add a few more zeros
    content[9000] = 0;

    fs::write(test_file, &content).expect("Failed to create dummy file");

    // We now have a file with 4 zero bytes.

    // --- Act: Run our function ---
    match count_zero_bytes(test_file) {
        Ok(count) => {
            println!("Found {} zero bytes in '{}'.", count, test_file.display());
            assert_eq!(count, 4); // Check our logic
        }
        Err(e) => {
            eprintln!("Error processing file '{}': {}", test_file.display(), e);
        }
    }

    // --- Cleanup ---
    fs::remove_file(test_file).ok();
}