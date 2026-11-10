// 06_fold_and_find_consumers.rs

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // --- 1. Use fold() to calculate the product ---
    // fold() is the most flexible consumer, reducing an iterator to a single value.
    // Starts with an initial accumulator value (1)
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product); // Output: 120

    println!("---");

    // --- 2. Use find() to get the first element satisfying a condition ---
    // find() returns an Option<&Item> (a reference to the found element).
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    
    match first_even {
        Some(n) => println!("First even number: {}", n), // Output: 2
        None => println!("No even numbers found."),
    }

    // --- 3. Getting the owned value after find() ---
    // Since find returns a reference, we use copied() to convert the Option<&i32> 
    // to an Option<i32> (the owned value) for easier use.
    let first_gt_3_value: Option<i32> = numbers.iter()
        .find(|&&x| x > 3) 
        .copied(); 
        
    println!("First > 3 value: {:?}", first_gt_3_value); // Output: Some(4)
    
    // Example of find returning None
    let first_gt_10_value: Option<i32> = numbers.iter()
        .find(|&&x| x > 10) 
        .copied();
    println!("First > 10 value: {:?}", first_gt_10_value); // Output: None
}