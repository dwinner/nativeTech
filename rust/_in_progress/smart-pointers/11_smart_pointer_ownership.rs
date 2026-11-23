// 11_smart_pointer_ownership.rs
use std::rc::Rc;

fn main() {
    println!("--- Part 1: Box<T> (Unique Ownership) ---");
    let b1 = Box::new(5);
    
    // Ownership MOVES from b1 to b2.
    // b1 is now invalid.
    let b2 = b1; 
    
    // println!("{}", b1); // Compile Error: Use of moved value
    println!("b2 owns the value: {}", b2);


    println!("\n--- Part 2: Rc<T> (Shared Ownership) ---");
    let rc1 = Rc::new("Shared Data".to_string());
    
    // Rc::clone creates a new pointer to the SAME data.
    // Ownership is now SHARED, not moved.
    let rc2 = Rc::clone(&rc1);

    // Both variables are valid.
    println!("rc1: {}", rc1);
    println!("rc2: {}", rc2);
    
    // The reference count confirms two owners exist.
    println!("Reference count: {}", Rc::strong_count(&rc1)); // Output: 2
}