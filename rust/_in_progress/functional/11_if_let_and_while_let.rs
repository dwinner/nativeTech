// 11_if_let_and_while_let.rs

fn main() {
    let maybe_value: Option<i32> = Some(10);

    // --- 1. if let: Concise handling of a single match case ---
    // This is equivalent to `match maybe_value { Some(value) => { ... }, _ => { ... } }`
    if let Some(value) = maybe_value {
        // 'value' is bound to 10 only if maybe_value is Some.
        println!("Got a value using if let: {}", value); 
    } else {
        println!("No value found.");
    }
    
    println!("---");

    // --- 2. while let: Loop until the pattern no longer matches ---
    let mut data_stack = vec![Some(1), Some(2), None, Some(3)];

    // Process items from the stack as long as they successfully match the nested pattern.
    while let Some(Some(value)) = data_stack.pop() {
        // data_stack.pop() returns Option<Option<i32>>
        // The loop continues only if:
        // 1. `pop()` returns `Some(...)` (stack is not empty) AND
        // 2. The popped value is `Some(...)` (it wasn't the `None` placeholder).
        println!("Processing value from stack: {}", value);
    }
    
    // The loop processed 3, 2, then stopped on None. The remaining item (Some(1)) 
    // was popped but the pattern didn't match the inner value, so the loop terminated.
    println!("Stack processing finished. Remaining: {:?}", data_stack); 
    // Note on output: The loop processed 3 and 2. The next item popped was None, stopping the loop.
    // The vector is reduced by the items popped: [Some(1)] remains.
}