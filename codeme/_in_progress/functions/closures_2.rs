fn main() {
    let my_name = String::from("Alice");
    let mut counter = 0;
    let data = vec![1, 2, 3];

    // 1. Captures `my_name` by immutable reference (&String) because it
    // only reads it.
    let greet = || println!("Hello, {}!", my_name);
    greet();
    // `my_name` is still valid here.
    println!("`my_name` can still be used: {}", my_name);

    // 2. Captures `counter` by mutable reference (&mut i32) because it
    // modifies it.
    let mut increment = || {
        counter += 1;
        println!("Counter is now: {}", counter);
    };
    increment();
    increment();
    // `counter` has been modified.
    println!("Final counter value: {}", counter);

    // 3. Captures `data` by taking ownership (Vec<i32>) because of the
    // `move` keyword.
    // We'll discuss `move` next.
    let consume_data = move || {
        println!("Consumed data: {:?}", data);
        // `data` is dropped when this closure ends.
    };
    consume_data();
    
    // The line below would cause a compile error because `data` was
    // moved.
    // println!("Can we use data after move? No: {:?}", data);
}