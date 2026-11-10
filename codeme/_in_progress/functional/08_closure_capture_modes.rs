// 08_closure_capture_modes.rs

fn main() {
    let factor = 10;
    let threshold = 50;

    // --- 1. Fn: Capture by Immutable Reference (&) ---
    // This closure captures 'factor' by immutable reference (&) because it only reads the variable.
    // It implements the Fn trait (can be called multiple times, non-mutating).
    let multiply_by_factor = |n| n * factor;
    println!("5 times factor: {}", multiply_by_factor(5)); // Output: 50
    // factor is still fully usable here:
    println!("Factor is: {}\n", factor); 

    // --- 2. FnMut: Capture by Mutable Reference (&mut) ---
    let mut items_processed = 0;
    
    // This closure captures 'items_processed' by mutable reference (&mut) because it modifies it. 
    // It must be marked 'mut' and implements the FnMut trait.
    let mut process_item = |item_id| {
        println!("Processing item {}", item_id);
        items_processed += 1; // Modifies captured variable
    };
    
    process_item(101);
    process_item(102);
    println!("Items processed: {}\n", items_processed); // Output: 2

    // --- 3. FnOnce: Capture by Value (Ownership/Move) ---
    let data_to_own = vec![1, 2, 3];
    
    // The 'move' keyword forces the closure to take ownership of 'data_to_own'.
    // It implements the FnOnce trait (can only be called once, as it consumes its captured variables).
    let consume_data = move || {
        println!("Consuming data: {:?}", data_to_own);
        // data_to_own is dropped when consume_data goes out of scope
    };
    
    consume_data();
    // ERROR! data_to_own was moved into the closure and is no longer valid in the main scope.
    // println!("{:?}", data_to_own); 
    
    println!("--- Iterator Capture ---");
    // Closures used with iterators often capture variables.
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let greater_than_threshold: Vec<i32> = numbers.into_iter() // into_iter() consumes numbers
        // Captures 'threshold' by immutable reference (&) because it only reads it.
        // Even though this is used inside a consuming iterator, the closure itself is only Fn.
        .filter(|&num| num > threshold) 
        .collect();
        
    // threshold is still usable here because filter only needed an immutable reference.
    println!("Threshold value: {}", threshold);
    println!("Numbers > {}: {:?}", threshold, greater_than_threshold); // Output: Numbers > 50: []
}