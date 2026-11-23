// 17_28_mismatched_return_types.rs

fn main() {
    // --- The Pitfall: Mismatched Return Types ---
    // The function below promises to return a heap-allocated `String` (owned data),
    // but strictly returns a string slice `&str` (a view into static memory).
    
    /*
    fn get_message() -> String {
        // Error: expected struct `String`, found `&str`
        "This is a static string slice" 
    }
    */

    // --- The Fix: Conversion ---
    let message = get_message_correctly();
    println!("{}", message);
}

fn get_message_correctly() -> String {
    let static_slice = "This is a static string slice";
    
    // We must convert the slice into an owned String to match the return signature.
    // Common ways: .to_string(), String::from(), or .into()
    static_slice.to_string() 
}