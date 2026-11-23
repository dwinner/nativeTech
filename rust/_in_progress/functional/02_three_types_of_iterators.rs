// 02_three_types_of_iterators.rs

fn main() {
    // --- 1. iter() - Immutable Borrows ---
    let names = vec!["Alice", "Bob", "Charlie"];
    
    // Creates an iterator over immutable references (`&T`).
    for name in names.iter() {
        // `name` here is a reference to the element (`&&str`).
        println!("Hello, {}!", name);
    }
    // `names` is still valid and can be used here because we only borrowed it.
    println!("The names vector is still available: {:?}\n", names);

    // --- 2. iter_mut() - Mutable Borrows ---
    let mut numbers = vec![10, 20, 30];
    
    // Creates an iterator over mutable references (`&mut T`).
    for num in numbers.iter_mut() {
        // `num` here is a mutable reference (`&mut i32`).
        // We must dereference `*num` to access and modify the original value.
        *num *= 2; 
    }
    // `numbers` has been modified in place.
    println!("The numbers vector has been modified: {:?}\n", numbers);

    // --- 3. into_iter() - Taking Ownership ---
    let messages = vec![String::from("First"), String::from("Second")];
    
    // Creates an iterator that consumes the collection, yielding owned values (`T`).
    for msg in messages.into_iter() {
        // `msg` here is the owned value (`String`). The String inside the vector is moved here.
        println!("Processing message: {}", msg);
    }
    
    // The `messages` vector has been moved into the loop and is no longer valid.
    // The line below would cause a compile-time error:
    // println!("Can we use messages again? No: {:?}", messages);
}