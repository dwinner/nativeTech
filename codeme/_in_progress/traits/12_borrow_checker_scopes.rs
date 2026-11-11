// 12_borrow_checker_scopes.rs

fn main() {
    println!("This code demonstrates lifetime scopes.");
    println!("Uncomment the block below to see the borrow checker error.");
}

// ---------------------------------------------------------------------------
// UNCOMMENT THE BLOCK BELOW TO SEE THE ERROR
// ---------------------------------------------------------------------------

/*
fn invalid_scope() {
    // The outer scope is lifetime 'a
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        // The inner scope is lifetime 'b
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
                          //  |       |
    }                     // -+       | <--- 'x' is dropped here!
                          //          |
    // We try to use 'r' here, but it points to memory that was freed at 'b
    println!("r: {}", r); //          |
}                         // ---------+
*/