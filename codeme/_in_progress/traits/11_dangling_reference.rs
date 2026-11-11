// 11_dangling_reference.rs

fn main() {
    println!("This example demonstrates a safety feature.");
    println!("Uncomment the code below to see the 'borrowed value does not live long enough' error.");
}

// ---------------------------------------------------------------------------
// UNCOMMENT THE FUNCTION BELOW TO SEE THE ERROR
// ---------------------------------------------------------------------------

/*
fn trigger_dangling_reference() {
    let reference_to_nothing;

    {
        let x = 5;
        
        // ERROR: 'x' only lives inside this block.
        // We are trying to borrow 'x' and store it in the outer variable.
        reference_to_nothing = &x; 
    
    } // 'x' is dropped here! The memory is freed.

    // If Rust allowed this, we would be pointing to invalid memory.
    // The compiler catches this and stops the build.
    println!("r: {}", reference_to_nothing); 
}
*/