use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    // std::fs::read_to_string returns a Result<String, std::io::Error>.
    // Since the file "file_no_exist.txt" almost certainly does not exist,
    // this will return an Err variant.
    let non_existent_file: Result<String, std::io::Error> =
        fs::read_to_string("file_no_exist.txt");

    println!("Attempting to read file...");
    
    // The .expect() method is uncommented here.
    // Since non_existent_file is an Err, this line will cause a panic.
    let _file_contents = non_existent_file.expect(
        "Expected the file to definitely exist! Check your file path."
    );

    // This line will NOT be reached because of the panic above.
    println!("Successfully read file!");
    
    Ok(())
}