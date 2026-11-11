// 17_05_lifetime_specifiers.rs

// --- The Pitfall: Missing Lifetimes ---
// This function fails to compile because Rust cannot determine if the returned 
// reference is borrowing from 'x' or 'y'. Therefore, it doesn't know how long 
// the return value is valid for.
/*
fn longest(x: &str, y: &str) -> &str { // Error: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
*/

// --- The Fix: Generic Lifetime Annotations ---
// We use the generic lifetime parameter 'a.
// This signature tells the compiler: "The returned reference is valid as long as 
// BOTH x and y are valid." effectively tying the output to the *shorter* of the inputs.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    
    // We declare `result` in the outer scope...
    let result;
    
    { // ...but we perform the operation in an inner scope.
        let string2 = String::from("xyz");
        
        // The lifetime 'a becomes the intersection of the lifetimes of string1 and string2.
        // Since string2 dies at the end of this block, the return value is valid 
        // ONLY within this block.
        result = longest(string1.as_str(), string2.as_str());
        
        println!("Inside scope: The longest string is {}", result); // This works
    } // `string2` is dropped here.

    // --- The Scope Constraint ---
    // Attempting to use `result` here causes a compile-time error.
    // Even though `result` *might* point to `string1` (which is still alive),
    // the compiler must assume the worst case (that it points to `string2`).
    
    // println!("Outside scope: {}", result); // Error: `string2` does not live long enough
}