// 17_04_dangling_references_scope.rs

fn main() {
    // --- The Pitfall: Scope Mismatch ---
    // In this scenario, we declare a reference 'r' in the outer scope,
    // but try to assign it to 'x', which only exists in the inner scope.
    
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // Error: `x` is borrowed here...
    // } // ...but `x` goes out of scope and is dropped here.
    
    // println!("r: {}", r); // 'r' would now be dangling (pointing to empty memory).

    // --- The Fix: Correct Scoping ---
    // Ensure the data being referenced (x) lives at least as long as the reference (r).
    
    let x = 5; // We move `x` to the outer scope.
    let r = &x; // Now `r` borrows from `x`, which shares the same scope.

    println!("r: {}", r); 
    // This works because `x` is not dropped until the end of main(), 
    // covering the entire lifetime of `r`.
}