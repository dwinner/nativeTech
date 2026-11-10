// 02_stack_vs_heap.rs

fn stack_example() {
    println!("--- Stack Example ---");
    // Stack: Fast allocation, fixed size at compile time.
    // 'x' and 'y' are stored directly on the stack frame for this function.
    let x = 10;      // x (an i32) is on the stack
    let y = true;    // y (a bool) is on the stack
    
    println!("x = {}, y = {}", x, y);
    println!("Values are popped off the stack when this function returns.");
    // When stack_example returns, x and y are simply popped off.
}

fn heap_example() {
    println!("\n--- Heap Example ---");
    // Heap: Slower allocation, dynamic size, more flexible.
    
    // String::from requests memory from the OS (the heap) to store "hello".
    // 's1' itself is a small struct (pointer, length, capacity) stored on the STACK.
    // It points to the actual data on the HEAP.
    let s1 = String::from("hello"); 

    // vec! macro allocates memory on the heap for [1, 2, 3].
    // 'v1' is the pointer/metadata on the stack.
    let v1 = vec![1, 2, 3]; 

    println!("String: '{}', Vector: {:?}", s1, v1);
    println!("When s1 and v1 go out of scope, their 'Drop' trait is called to free the heap memory.");
}

fn main() {
    println!("Demonstrating Stack vs Heap memory in Rust:\n");
    stack_example();
    heap_example();
}