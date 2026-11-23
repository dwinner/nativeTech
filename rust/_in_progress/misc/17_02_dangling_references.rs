// 17_02_dangling_references.rs

fn main() {
    // --- The Pitfall: Dangling Reference ---
    
    // The following line would cause a compile-time error because 'dangle' tries 
    // to return a reference to a value that is dropped at the end of the function.
    // let reference_to_nothing = dangle(); 

    // Rust prevents this to avoid pointing to deallocated memory.
    
    // --- The Fix: Transferring Ownership ---
    
    // Instead of returning a reference, we return the String itself.
    // This moves ownership out of the function to 'owned_string'.
    let owned_string = no_dangle();
    println!("We own the string: {}", owned_string);
}

// This function is invalid and will not compile if uncommented.
/*
fn dangle() -> &String { 
    let s = String::from("hello"); 
    &s // Error: 's' is dropped here, so the reference would point to invalid memory.
}
*/

// This function works by returning the value directly.
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved to the caller
}