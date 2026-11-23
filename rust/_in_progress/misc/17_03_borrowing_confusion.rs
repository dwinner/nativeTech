// 17_03_borrowing_confusion.rs

fn main() {
    let mut s = String::from("hello");

    // --- The Pitfall: Overlapping Borrows ---
    
    let r1 = &s; // Immutable borrow starts here
    
    // The following line causes an error because we try to create a mutable borrow (r2)
    // while the immutable borrow (r1) is still active (it is used in the println below).
    // let r2 = &mut s; 
    
    // println!("r1: {}, r2: {}", r1, r2); 

    println!("r1 (safe use): {}", r1); 
    // r1's effective lifetime ends here under Non-Lexical Lifetimes (NLL) 
    // because it is not used again.

    // --- Fix 1: Explicit Scoping ---
    // We can use a block to force the immutable borrow to go out of scope explicitly.
    {
        let r1_scoped = &s;
        println!("r1_scoped: {}", r1_scoped);
    } // r1_scoped drops here; the borrow ends.

    let r2 = &mut s; // Now it is safe to borrow mutably
    r2.push_str(", world");
    println!("r2: {}", r2);

    // --- Fix 2: Ordering (Non-Lexical Lifetimes) ---
    // Ensure the last use of the immutable borrow happens BEFORE the mutable borrow.
    let mut data = String::from("original");
    
    let imm_borrow = &data;
    println!("Immutable borrow: {}", imm_borrow);
    // This is the last use of 'imm_borrow'. The compiler sees it's no longer needed.
    
    let mut_borrow = &mut data; // Valid, because imm_borrow is no longer active
    mut_borrow.push_str(" changed");
    println!("Mutable borrow: {}", mut_borrow);
}