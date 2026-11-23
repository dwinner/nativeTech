// 17_29_unresolved_imports.rs
use std::time::Instant; // Fix: explicit import required

fn main() {
    // --- The Pitfall: Unresolved Import ---
    // `Instant` is NOT in the Rust "prelude" (the list of things available everywhere).
    // Therefore, using it without an import causes a compile-time error.
    
    // let start_time = Instant::now(); 
    // Error: failed to resolve: use of undeclared type or module `Instant`

    // --- The Fix: Import or Qualify ---
    // Option 1: Use the import defined at the top (use std::time::Instant;)
    let start_time = Instant::now();
    
    // Simulate some work
    // Note: We can also use the fully qualified path without a top-level import:
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
}