fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &element) in arr.iter().enumerate() {
        if element == target {
            return Some(index); // Found it, return Some(index)
        }
    }
    None // Didn't find it, return None
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Case 1: The element is found
    match find_element(&numbers, 3) {
        Some(index) => println!("Using match: Found element at index: {}",
index),
        None => println!("Using match: Element not found"),
    }

    // Case 2: The element is not found
    match find_element(&numbers, 6) {
        Some(index) => println!("Using match: Found element at index: {}",
index),
        None => println!("Using match: Element not found"),
    }
}