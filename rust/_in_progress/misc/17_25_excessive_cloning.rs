// 17_25_excessive_cloning.rs

#[derive(Clone)]
struct LargeData {
    id: u32,
    // A large vector to simulate heavy memory usage (1MB)
    payload: Vec<u8>, 
}

fn main() {
    // Setup: Create a heavy object
    let data = LargeData {
        id: 1,
        payload: vec![0u8; 1024 * 1024],
    };

    // --- The Pitfall: Excessive Cloning ---
    // The function `process_owned` takes full ownership.
    // If we want to use `data` again later, we are FORCED to clone it.
    // This performs a "Deep Copy", duplicating the entire 1MB payload on the heap.
    
    // process_owned(data.clone()); // Slow: Allocation + Copy
    
    // println!("Original data ID is still here: {}", data.id);

    // --- The Fix: Pass by Reference (Borrowing) ---
    // The function `process_borrowed` takes a reference (&LargeData).
    // We pass a pointer to the existing data. No new heap allocation occurs.
    
    process_borrowed(&data); // Fast: Pointer copy only
    
    println!("Original data ID is still here: {}", data.id);
}

// Bad Practice: Forces a move (or clone), destroying the original unless cloned
fn process_owned(d: LargeData) {
    println!("Processing owned ID: {}", d.id);
    // 'd' is dropped here, deallocating the cloned memory
}

// Good Practice: Borrows the data for reading
fn process_borrowed(d: &LargeData) {
    println!("Processing borrowed ID: {}", d.id);
}