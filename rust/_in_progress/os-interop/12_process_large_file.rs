// 12_process_large_file.rs

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::path::Path;

const CHUNK_SIZE: usize = 8 * 1024; // 8KB chunks

fn process_large_file_in_chunks(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file); // Buffering is still good!
    let mut chunk = vec![0u8; CHUNK_SIZE];

    loop {
        // Attempt to fill the chunk buffer
        // reader.read() might not fill the whole buffer if EOF is reached.
        let bytes_read = match reader.read(&mut chunk) {
            Ok(0) => break, // End of file
            Ok(n) => n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue, // Retry on interrupt
            Err(e) => return Err(e), // Other error
        };

        // Process the data in `&chunk[..bytes_read]`
        // For example, count occurrences of a byte, hash the chunk, etc.
        println!("Processed a chunk of {} bytes", bytes_read);

        // In a real app, you'd do something more useful here.

        if bytes_read < CHUNK_SIZE {
            break; // Likely reached EOF or a partial last chunk
        }
    }
    Ok(())
}

fn main() {
    // You'd need a large_test_file.dat for this to be meaningful
    // For now, let's just show the function definition and conceptual call

    // if let Err(e) = process_large_file_in_chunks(Path::new("large_test_file.dat")) {
    //     eprintln!("Error processing large file: {}", e);
    // }

    println!("Conceptual chunked file processing function defined.");
}