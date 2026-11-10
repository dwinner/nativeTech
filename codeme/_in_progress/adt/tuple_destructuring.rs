fn main() {
    // 1. Define the tuple with a fixed set of heterogeneous types.
    let tuple: (i32, f64, bool) = (42, 6.7, true);

    // 2. Destructuring: The pattern (int_value, float_value, bool_value) 
    // matches the structure of the tuple on the right, extracting the values.
    let (int_value, float_value, bool_value) = tuple;

    println!("--- Destructured Values ---");
    println!("Integer value: {}", int_value);
    println!("Float value: {}", float_value);
    println!("Boolean value: {}", bool_value);

    // Because i32, f64, and bool are all Copy types, the original 'tuple' 
    // variable is still valid after destructuring.
    println!("Original tuple value: {}", tuple.0); 
}