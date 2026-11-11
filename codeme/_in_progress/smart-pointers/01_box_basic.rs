// 01_box_basic.rs

// It's standard practice to define structs outside the main function.
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Create a Box<i32> to store an integer on the heap.
    // Instead of living on the stack, the number 5 is stored on the heap,
    // and 'heap_int' holds the pointer to it.
    let heap_int = Box::new(5);
    println!("Value in a Box: {}", heap_int);

    // Create a Box<Point> to store a struct on the heap.
    let heap_point = Box::new(Point { x: 10.0, y: 20.5 });
    println!("Struct in a Box: {:?}", heap_point);

    // Explicitly dereference the Box to access the value.
    // The '*' operator follows the pointer to get the actual data.
    let value_from_box = *heap_int;
    println!("Explicitly dereferenced value: {}", value_from_box);

    // Access a field directly thanks to automatic dereferencing (Deref coercion).
    // Rust automatically treats `heap_point.x` as `(*heap_point).x`.
    println!("Accessing field via deref coercion: heap_point.x = {}", heap_point.x);

    // When `heap_int` and `heap_point` go out of scope here, the memory they manage
    // on the heap is automatically freed via the Drop trait.
}