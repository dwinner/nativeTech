// 05_filter_and_chaining.rs

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Create an iterator, filter for even numbers, and collect.
    let evens: Vec<i32> = numbers.iter() // Iterator yields &i32
        // filter() closure must return a boolean.
        // We use ||x| *x % 2 == 0 OR ||&&x| x % 2 == 0 for clarity, depending on how you destructure.
        // We use .copied() to convert &i32 to owned i32 for the final vector.
        .filter(|&x| x % 2 == 0) 
        .copied() // Convert the iterator of &i32 to i32
        .collect(); // Collects the i32 results
        
    println!("Original: {:?}", numbers); // Output: [1, 2, ..., 10]
    println!("Evens: {:?}", evens); // Output: [2, 4, 6, 8, 10]

    println!("---");
    
    // Chaining map and filter
    let scores = vec![85, 42, 95, 60, 77];
    
    // Get scores above 70, and add a 5-point bonus
    let adjusted_high_scores: Vec<i32> = scores.iter()
        // 1. Filter: Keep scores > 70
        .filter(|&score| score > 70) 
        // 2. Map: Add 5 to the filtered scores
        .map(|&score| score + 5) 
        // 3. Collect: Execute the operations and build the final Vec
        .collect();
        
    println!("Adjusted high scores: {:?}", adjusted_high_scores); // Output: [90, 100, 82]
}