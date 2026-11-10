// 17_09_scope_confusion.rs

fn main() {
    // --- The Pitfall: Scope Confusion ---
    // An immutable borrow in an outer scope remains active if it is used
    // after an inner scope where a mutable borrow is attempted.
    
    /*
    let mut s = String::from("hello");
    let r1 = &s; // r1 lifetime starts here
    
    {
        // Error: cannot borrow `s` as mutable because it is also borrowed as immutable by `r1`.
        // Even though this is an inner scope, r1 is still "live" because of the println below.
        let r2 = &mut s; 
        r2.push_str(", world");
    }
    
    println!("r1 after inner scope: {}", r1); // This usage extends r1's lifetime
    */

    // --- Fix 1: Explicit Scoping ---
    // Ensure the immutable borrow dies completely before the mutable borrow starts.
    let mut s = String::from("hello");
    
    {
        let r1_scoped = &s;
        println!("r1 in its own scope: {}", r1_scoped);
    } // r1_scoped drops here.

    // Now, the mutable borrow is fine in a subsequent scope.
    {
        let r2_inner = &mut s; 
        r2_inner.push_str(", world");
        println!("s after modification: {}", s);
    }

    // --- Fix 2: Re-borrowing (Ordering) ---
    // If you need to read the value before and after mutation:
    // Borrow -> Drop (stop using) -> Mutate -> Re-borrow.
    let mut s2 = String::from("another example");
    
    let r1 = &s2;
    println!("r1 before change: {}", r1);
    // Last use of r1. Under Non-Lexical Lifetimes (NLL), its borrow ends here.

    let r2 = &mut s2;
    r2.push_str(" with changes");
    println!("s2 changed: {}", r2); // r2 usage ends here.

    // If we need an immutable reference again, we create a NEW one.
    // We cannot use 'r1' here because the data it pointed to has changed 
    // and the previous borrow was invalidated by the mutable borrow.
    let r3 = &s2;
    println!("s2 via r3: {}", r3);
}