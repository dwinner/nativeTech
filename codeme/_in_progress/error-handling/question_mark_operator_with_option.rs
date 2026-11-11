// A function that finds the first number in a slice satisfying a condition
// and possibly performs an operation on it.
// Returns Option<i32> as the final result of the operation.
fn find_and_operate(slice: &[i32], operation: fn(i32) -> Option<i32>) -> Option<i32> {
    // 1. Find the first element > 10. 
    // .find() returns Option<&i32>. 
    // '?' propagates None if not found, otherwise it unwraps to &i32.
    let found_element = slice.iter().find(|&&x| x > 10)?; 
    
    // Dereference found_element (which is &i32) to get i32, and perform the operation.
    let operation_result = operation(*found_element)?; 
    
    // 3. If both steps succeeded (didn't return None), return the result wrapped in Some.
    Some(operation_result) 
}

// Example operation: double if even, otherwise None
// This function returns None if the number is odd.
fn double_if_even(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n * 2)
    } else {
        None
    }
}

fn main() {
    let numbers1 = [5, 12, 8, 15, 6]; // First > 10 is 12 (even)
    let numbers2 = [5, 8, 13, 9];    // First > 10 is 13 (odd)
    let numbers3 = [1, 2, 3];        // No number > 10

    // Test Case 1: Success (12 is found, 12 * 2 = 24 is returned)
    let result1 = find_and_operate(&numbers1, double_if_even);
    println!("Operation result on numbers1: {:?}", result1); // Output: Some(24)

    // Test Case 2: Operation Fails (13 is found, but double_if_even returns None)
    let result2 = find_and_operate(&numbers2, double_if_even);
    println!("Operation result on numbers2: {:?}", result2); // Output: None 
    
    // Test Case 3: Find Fails (No number > 10 is found)
    let result3 = find_and_operate(&numbers3, double_if_even);
    println!("Operation result on numbers3: {:?}", result3); // Output: None 
}