// This function safely returns two string slices (&str) that borrow from the input string.
fn split_name(full_name: &str) -> (&str, &str) {
    // Find the index of the first space
    let space_index = full_name.find(' ').unwrap_or(full_name.len());
    
    // Slice the string:
    let first = &full_name[..space_index];
    let last = &full_name[space_index + 1..];
    
    // Return the two slices as a tuple
    (first, last)
}

fn main() {
    // The literal "John Doe" is a string slice (&str), which is borrowed by the function.
    let full_name = "John Doe";
    
    // Destructure the returned tuple into first_name and last_name.
    let (first_name, last_name) = split_name(full_name);

    println!("--- Name Split Results ---");
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
    
    // Test with a different name
    let long_name = "Jane Elizabeth Smith"; 
    let (first_part, remainder) = split_name(long_name);
    println!("\nLong Name Split (First Part): {}", first_part);
    println!("Long Name Split (Remainder): {}", remainder);
}