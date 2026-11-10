// 04_c_callbacks.rs

// Rust function to be called by C
// It must have the C calling convention and be `unsafe` if it does unsafe things,
// or be safe if its operations are all safe.
// For C to call it, it needs to be `extern "C"`.

// UPDATE: We use #[unsafe(no_mangle)] because messing with linker symbols is unsafe.
#[unsafe(no_mangle)] 
pub extern "C" fn rust_callback_function(value: i32) {
    println!("[Rust Callback] Called from C with value: {}", value);
}

// Assume a C function like this exists:
// typedef void (*rust_callback_t)(int);
// void register_and_call_rust_callback(rust_callback_t cb, int data_for_cb);

// extern "C" {
    // For this example, we'll only declare it conceptually.
    // fn register_and_call_rust_callback(
    //     callback: extern "C" fn(i32), // Type for a function pointer
    //     data: i32
    // );
// }

fn main() {
    println!("Conceptual example of providing a Rust callback to C.");
    println!("To run this properly, you'd usually have a C library linking to this.");
    
    // --- Simulation ---
    // Since we don't have a real C library linked here, let's simulate 
    // what the C side would do: create a function pointer and call it.
    
    println!("\n--- Simulating C side ---");
    
    // This is the type a C function pointer would map to in Rust
    let c_side_pointer: extern "C" fn(i32) = rust_callback_function;

    println!("C is about to call the callback with data: 42");
    
    // Call the function via the pointer
    c_side_pointer(42);
}