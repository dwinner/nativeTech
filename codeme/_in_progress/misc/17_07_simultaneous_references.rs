// 17_07_simultaneous_references.rs

fn main() {
    // --- The Pitfall: Simultaneous Mutable and Immutable References ---
    /*
    let mut s = String::from("hello");
    let r1 = &s; // Immutable borrow starts here
    
    // Attempting to create a mutable borrow while r1 is live:
    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    let r2 = &mut s; 
    
    println!("Immutable: {}", r1); // r1 is used here, so its borrow is active.
    // println!("Mutable: {}", r2);
    */

    // --- Fix: Option 1 - Using a specific scope ---
    // Ensure the lifetime of the immutable reference ends before the mutable reference is created.
    let mut s = String::from("hello");
    {
        let r1 = &s; // Immutable borrow in a new scope
        println!("r1 in its scope: {}", r1);
    } // r1 goes out of scope here; its borrow ends.

    let r2 = &mut s; // Now we can create a mutable borrow.
    r2.push_str(", world");
    println!("s after modification: {}", s);

    // --- Fix: Option 2 - Leveraging Non-Lexical Lifetimes (NLL) ---
    // The compiler knows that r1_nll is not used after the println, so it ends the borrow there.
    let mut s_nll = String::from("NLL example");
    
    let r1_nll = &s_nll;
    println!("r1_nll first use: {}", r1_nll); // Last use of r1_nll. Its borrow can end here.
    
    let r2_nll = &mut s_nll; // This is allowed because r1_nll is no longer used.
    r2_nll.push_str(" works!");
    println!("r2_nll: {}", r2_nll);
}