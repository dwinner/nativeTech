// 17_01_double_ownership.rs

fn main() {
    // --- The Pitfall: Move Semantics ---
    let s1 = String::from("hello");
    
    // Ownership of the heap data is moved from s1 to s2
    let s2 = s1; 
    
    // If we uncomment the line below, the compiler throws an error 
    // because s1 is no longer valid to prevent "double free" issues.
    // println!("s1 is: {}", s1); // Error: borrow of moved value: `s1`
    
    println!("s2 now owns the data: {}", s2);

    // --- The Fix: Cloning ---
    let s3 = String::from("world");
    
    // We explicitly create a deep copy of the heap data for s4
    let s4 = s3.clone(); 
    
    // Both variables remain valid because they point to independent data
    println!("s3 is still valid: {}", s3); 
    println!("s4 is a copy: {}", s4);
    
    // --- Note on Copy Types ---
    let x = 5;
    let y = x; // Integers implement the Copy trait, so no move occurs
    
    println!("x is still valid: {}, and y is: {}", x, y);
}