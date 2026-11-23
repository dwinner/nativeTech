fn main() {
    let mut greeting = String::from("Hello"); // `greeting` owns the String
    {
        let read1 = &greeting; // Immutable borrow: Can read `greeting`
        let read2 = &greeting; // Another immutable borrow: OK to have many
        // readers
        println!("Immutable read 1: {}", read1);
        println!("Immutable read 2: {}", read2);
    } // `read1` and `read2` are no longer used after this block
      // `read1` and `read2` go out of scope here, so they're no longer
      // borrowing `greeting`
    
    let mut write1 = &mut greeting; // Mutable borrow: Can modify `greeting`
    write1.push_str(", Rust!"); // Modify the string
    println!("Mutable write: {}", write1);
    
    println!("Original greeting: {}", greeting); // `greeting` is still valid
}