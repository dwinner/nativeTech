// Function returning (String, u32) tuple
fn get_server_info() -> (String, u32) {
    // This tuple contains a non-Copy type (String) and a Copy type (u32)
    (String::from("192.168.1.100"), 8080)
}

fn main() {
    // 1. Define tuples (from previous image)
    let basic_tuple: (i32, f64, bool) = (100, 3.14, true);
    let mix_tuple = ("Rust", 2015, '🦀'); // Type: (&str, i32, char)

    println!("--- 1. Direct Indexing ---");
    // Access elements using dot notation and the zero-based index.
    let first_val = basic_tuple.0; // 100 (i32)
    let second_val = basic_tuple.1; // 3.14 (f64)
    println!("First: {}, Second: {}", first_val, second_val);
    
    println!("\n--- 2. Destructuring ---");
    // Destructure the tuple into individual variables using a pattern.
    let (language, year, mascot) = mix_tuple;
    println!("Language: {}, Year: {}, Mascot: {}", language, year, mascot);
    // Note: Since mix_tuple only contained Copy types (&str, i32, char), 
    // the original mix_tuple variable is still usable, but if it contained 
    // a String, ownership would have moved.

    println!("\n--- 3. Returning Multiple Values ---");
    // Call the function and immediately destructure the returned tuple.
    let (ip_address, port) = get_server_info();
    println!("Attempting to connect to server: {}:{}", ip_address, port);
    // Note: The variable 'ip_address' now owns the String data returned by the function.
    
    // Demonstrate basic printing (from previous image)
    println!("\n--- Basic Printing ---");
    println!("Basic tuple: {:?}", basic_tuple);
    println!("Mixed tuple: {:?}", mix_tuple); 
}