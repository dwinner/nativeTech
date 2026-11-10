// 17_08_multiple_mutable_references.rs

fn main() {
    // --- The Pitfall: Multiple Mutable References ---
    // Rust prevents having more than one mutable reference to the same data 
    // at the same time to avoid data races.
    
    /*
    let mut s = String::from("hello");
    let r1 = &mut s; // First mutable borrow
    
    // Error: cannot borrow `s` as mutable more than once at a time
    let r2 = &mut s; 
    
    r1.push_str(", world");
    r2.push_str("!"); 
    
    println!("{}", s);
    */

    // --- The Fix: Sequential Scopes ---
    // We structure the code so that mutable borrows happen one after another, 
    // rather than simultaneously.
    
    let mut s = String::from("hello");

    {
        let r1 = &mut s; // First mutable borrow starts here
        r1.push_str(", world");
        println!("After r1: {}", r1); 
    } // r1 goes out of scope here; its mutable borrow ends.

    {
        let r2 = &mut s; // Second mutable borrow is now allowed
        r2.push_str("!");
        println!("After r2: {}", r2); 
    } // r2 goes out of scope here.

    println!("Final string: {}", s);
}