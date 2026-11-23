// 03_iterator_consumers.rs

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // --- Using collect() ---
    // Collect is extremely versatile, turning an iterator into any collection type.
    // We use a type annotation `Vec<_>` to tell collect to build a Vec.
    let greater_than_two: Vec<_> = numbers
        .iter()
        // filter() is an "adapter" that lazily filters the sequence.
        // The numbers.iter() yields `&i32` (references), so we must dereference twice `&&n` 
        // to get the actual value `n`.
        .filter(|&&n| n > 2)
        // collect() is a "consumer" that executes all previous operations.
        .collect();
    
    println!("Numbers greater than 2: {:?}", greater_than_two); // Output: [3, 4, 5]

    // --- Using sum() ---
    // sum() is a consumer that adds up all elements.
    let total: i32 = (1..=10).sum(); // The iterator is the range (1..=10)
    println!("The sum of numbers from 1 to 10 is: {}", total); // Output: 55

    // --- Using fold() ---
    // fold() is the most flexible consumer, reducing an iterator to a single value.
    // It takes an initial accumulator value (1) and a closure.
    let product = numbers
        .iter()
        // |accumulator, &item|: The accumulator holds the running total/product.
        // &item is used because numbers.iter() yields references.
        .fold(1, |accumulator, &item| accumulator * item);
    
    println!("The product of the numbers is: {}", product); // Output: 120
}