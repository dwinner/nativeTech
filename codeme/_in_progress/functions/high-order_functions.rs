fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Let's find the sum of the squares of all even numbers greater than
    // 3.
    let result: i32 = numbers
        .iter() // Create an iterator that yields references (&i32)
        
        // Use a closure with `filter` to keep only even numbers greater
        // than 3. 
        // `n` here is `&&i32`, so we need to dereference it twice.
        .filter(|& &n| n > 3 && n % 2 == 0)

        // Use a closure with `map` to square each remaining number.
        // `n` here is `&i32`, so we dereference it once.
        .map(|&n| n * n)

        // Use `sum()` to consume the iterator and add up the results.
        .sum();

    // The chain of operations would be:
    // Original: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // After filter: [4, 6, 8, 10]
    // After map: [16, 36, 64, 100]
    // After sum: 216

    println!("The sum of the squares of even numbers greater than 3 is: {}", result);
}