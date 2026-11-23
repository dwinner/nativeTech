// 17_06_mutable_vs_immutable_lifetimes.rs

fn main() {
    // --- The Pitfall: Overlapping Lifetimes ---
    /*
    let mut s = String::from("hello");
    let r1 = &s; // Immutable borrow starts.
    
    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable by r1.
    // The compiler sees that r1 is used in the println! below, so its lifetime must extend to there.
    let r2 = &mut s; 
    
    println!("r1 is: {}", r1); // Usage extends lifetime of r1
    // println!("r2 is: {}", r2);
    */

    // --- Fix 1: Explicit Scoping ---
    // We create a new block to force the immutable borrow to end early.
    let mut s = String::from("hello");
    {
        let r1 = &s;
        println!("r1 in its scope: {}", r1);
    } // `r1` goes out of scope here; the borrow ends completely.

    let r2 = &mut s; // Now, a mutable borrow is safe.
    r2.push_str(", world");
    println!("s after modification: {}", s);

    // --- Fix 2: Leveraging Non-Lexical Lifetimes (NLL) ---
    // Rust's borrow checker is smart enough to see that a reference is no longer used
    // and end its borrow early, even without explicit brackets.
    let mut s_nll = String::from("NLL example");
    
    let r1_nll = &s_nll;
    println!("r1_nll: {}", r1_nll); 
    // This is the LAST use of r1_nll. 
    // Under NLL, the compiler decides the borrow ends right here.

    let r2_nll = &mut s_nll; // This is now allowed!
    r2_nll.push_str(" works!");
    println!("r2_nll: {}", r2_nll);
}