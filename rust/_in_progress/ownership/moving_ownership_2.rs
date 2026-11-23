// A function that takes ownership of a large vector (a "move").
// This is very fast because only the pointer, length, and capacity are
// moved.
fn process_large_data_by_move(data: Vec<i32>) {
    // Imagine some processing happens here.
    println!("Processing data by move. First element: {}, Length: {}",
    data[0], data.len());
} // data is dropped here, and its heap memory is freed.

// A function that takes a clone of a large vector.
// This is much slower because it must allocate new memory and copy all
// elements.
fn process_large_data_by_clone(data: &Vec<i32>) {
    let data_clone = data.clone();
    println!("Processing data by clone. First element: {}, Length: {}",
    data_clone[0], data_clone.len());
} // data_clone is dropped here, and its heap memory is freed.

fn main() {
    // Create a large vector with one million integers.
    let large_vector: Vec<i32> = (0..1_000_000).collect();
    // Create another one for the clone example.
    let another_large_vector: Vec<i32> = (0..1_000_000).collect();

    println!("--- Demonstrating a move ---");
    // Ownership of `large_vector` is moved to the function. This is a
    // cheap operation.
    process_large_data_by_move(large_vector);
    // `large_vector` is no longer valid here and cannot be used.
    // println!("Can we use large_vector again? No."); // This would be a
    // compile error.

    println!("\n--- Demonstrating a clone ---");
    // We pass a reference and clone it inside the function.
    // This is an expensive operation, as it copies 1,000,000 integers.
    process_large_data_by_clone(&another_large_vector);
    // `another_large_vector` is still valid here because it was only
    // borrowed.
    println!("We can still use another_large_vector again. Length: {}",
    another_large_vector.len());
}