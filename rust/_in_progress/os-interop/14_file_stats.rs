// 14_file_stats.rs
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process;

#[derive(Debug, Default)]
struct FileStats {
    lines: usize,
    words: usize,
    chars: usize,
}

fn main() -> io::Result<()> {
    // 1. Get file path from command-line arguments.
    let file_path_str = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: my_wc <file_path>");
            process::exit(1);
        }
    };

    let file_path = Path::new(&file_path_str);

    // 2. Open the file and prepare to read it line by line.
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // 3. Process the file to gather stats.
    let mut stats = FileStats::default();

    for line_result in reader.lines() {
        let line = line_result?; // Propagate I/O error if reading a line fails
        stats.lines += 1;
        stats.words += line.split_whitespace().count();
        stats.chars += line.chars().count();
    }

    // 4. Get total file size in bytes from metadata for accuracy.
    let file_size_bytes = fs::metadata(file_path)?.len();

    // 5. Print the results.
    println!("\n--- Statistics for '{}' ---", file_path.display());
    println!(" Lines:      {}", stats.lines);
    println!(" Words:      {}", stats.words);
    println!(" Characters: {}", stats.chars);
    println!(" Bytes:      {}", file_size_bytes);

    Ok(())
}