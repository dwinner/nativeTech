// 04_map_iterator_adapter.rs

fn main() {
    let numbers = vec![1, 2, 3, 4];
    
    // Create an iterator, map a closure to square each number,
    // and then collect the results.
    let squares: Vec<i32> = numbers.iter() // Iterator yields &i32
        // The map closure takes the item by reference (&x) and returns the squared i32.
        // Rust automatically handles the dereference/copy for integer multiplication here.
        .map(|x| x * x) 
        .collect(); // Collects the i32 results
        
    println!("Original: {:?}", numbers); // Output: [1, 2, 3, 4]
    println!("Squares: {:?}", squares); // Output: [1, 4, 9, 16]

    println!("---");

    // Example with Strings
    let names = vec!["alice", "bob", "charlie"];
    
    let upper_names: Vec<String> = names.iter() // Iterator yields &&str (reference to string slice)
        // The closure uses a method that consumes the string slice and returns an owned String.
        .map(|name| name.to_uppercase()) 
        .collect(); // Collects the owned Strings

    println!("Original names: {:?}", names);
    println!("Upper names: {:?}", upper_names); // Output: ["ALICE", "BOB", "CHARLIE"]
}