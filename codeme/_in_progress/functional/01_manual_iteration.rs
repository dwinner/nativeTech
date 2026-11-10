// 01_manual_iteration.rs

fn main() {
    let fruits = vec!["apple", "banana", "cherry"];

    // Create an iterator over references to the elements in the vector.
    // The iterator variable must be `mut` because calling `.next()` advances
    // the iterator's internal state.
    let mut fruit_iterator = fruits.iter();

    // The Iterator trait requires the next() method, which returns an Option<Item>.
    println!("First call:  {:?}", fruit_iterator.next()); // Some("apple")
    
    println!("Second call: {:?}", fruit_iterator.next()); // Some("banana")
    
    println!("Third call:  {:?}", fruit_iterator.next()); // Some("cherry")
    
    // When there are no items left, it returns None.
    println!("Fourth call: {:?}", fruit_iterator.next()); // None 

    // Once exhausted, it continues to return None.
    println!("Fifth call:  {:?}", fruit_iterator.next()); // None
}