// 03_raw_pointers.rs

fn main() {
    let mut num = 5;

    // Create raw pointers from references. This part is safe.
    // r1 is an immutable raw pointer to num.
    let r1: *const i32 = &num as *const i32;
    // r2 is a mutable raw pointer to num.
    let r2: *mut i32 = &mut num as *mut i32;

    // --- WARNING: DANGEROUS OPERATION ---
    // The following code creates a raw pointer from an arbitrary memory address.
    // This is NOT something you should do in normal application code.
    // Accessing random memory addresses is undefined behavior and will
    // almost certainly crash your program with a segmentation fault.
    // This is only done in very specific low-level programming, like
    // interacting with known, fixed hardware addresses.
    let arbitrary_address = 0x012345usize;
    let r3_arbitrary_ptr = arbitrary_address as *const i32;
    // --- END WARNING ---

    // To dereference raw pointers and access the data they point to,
    // we MUST use an `unsafe` block. We are telling the compiler
    // that we take responsibility for the pointer's validity at this
    // moment.
    unsafe {
        // Dereferencing r1 to read the value of num
        // This is safe because we know r1 was created from a valid reference.
        println!("Value via r1 (immutable raw pointer): {}", *r1); // Output: 5

        // Dereferencing r2 to write a new value to the memory location of num.
        // This is safe because r2 was also created from a valid reference.
        *r2 = 10; // Modifies `num` through the raw pointer
        println!("`num` has been changed via r2 to: {}", num); // Output: 10

        // r1 still points to `num`, so it will now see the new value.
        println!("Value via r1 after change via r2: {}", *r1); // Output: 10

        // --- DANGER: DO NOT DO THIS ---
        // Uncommenting the line below would attempt to dereference r3_arbitrary_ptr.
        // This is EXTREMELY DANGEROUS because r3_arbitrary_ptr points to an arbitrary,
        // likely invalid, memory location. It would almost certainly crash your program.
        // println!("Attempting to read from arbitrary address r3: {}", *r3_arbitrary_ptr);
        // --- END DANGER ---
    }

    // Creating a null pointer.
    let null_pointer: *const i32 = std::ptr::null();
    let _mut_null_pointer: *mut i32 = std::ptr::null_mut();

    // It's crucial to check if a raw pointer is null before attempting to dereference it.
    if !null_pointer.is_null() {
        // This block will not execute because null_pointer is indeed null.
        unsafe {
            println!("This line should not be reached: {}", *null_pointer);
        }
    } else {
        println!("null_pointer is confirmed to be null, not dereferencing.");
    }
}