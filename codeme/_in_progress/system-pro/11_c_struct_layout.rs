// 03_c_struct_layout.rs
use std::os::raw::{c_int, c_double, c_char};

// Suppose you have a C struct like this:
// typedef struct {
//    int id;
//    double value;
//    char active; // Assuming char is used as a boolean (0 or 1)
// } CItem;

// The equivalent Rust struct with #[repr(C)]
#[repr(C)] // Ensure C-compatible memory layout
#[derive(Debug, Copy, Clone)] // Optional, for convenience
pub struct RustItemEquivalent {
    id: c_int,       // Use c_int for C int
    value: c_double, // Use c_double for C double
    active: c_char,  // Use c_char for C char
}

// Assume a C function:
// void process_c_item(const CItem* item_ptr);

// For this example, let's simulate it in Rust to make it runnable.
// We use extern "C" to tell the compiler to use the C calling convention for this function.
unsafe extern "C" fn process_c_item(item_ptr: *const RustItemEquivalent) {
    if item_ptr.is_null() {
        println!("[Simulated C] Received a null item_ptr!");
        return;
    }

    // In unsafe block because we dereference a raw pointer.
    let item_ref = &*item_ptr; // Dereference to get a Rust reference
    
    println!("[Simulated C] Processing item - ID: {}, Value: {}, Active: {}",
             item_ref.id, 
             item_ref.value, 
             if item_ref.active != 0 { "yes" } else { "no" }
    );
}

fn main() {
    let my_item = RustItemEquivalent {
        id: 101,
        value: 3.14159,
        active: 1, // Representing true for C char
    };

    let my_item_inactive = RustItemEquivalent {
        id: 102,
        value: 2.718,
        active: 0, // Representing false
    };

    // Pass a pointer to our Rust struct to the (simulated) C function.
    // This must be in an unsafe block because process_c_item is extern "C".
    unsafe {
        println!("Calling C function with my_item:");
        process_c_item(&my_item as *const RustItemEquivalent);

        println!("\nCalling C function with my_item_inactive:");
        process_c_item(&my_item_inactive as *const RustItemEquivalent);
    }
}