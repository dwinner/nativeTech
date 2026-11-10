// 13_lifetime_syntax.rs

// This function demonstrates the syntax.
// We must declare <'a> before we can use it in the arguments.
fn syntax_demo<'a>(
    // 1. A regular reference
    // The compiler infers the lifetime here (standard usage).
    x: &i32, 

    // 2. A reference with an explicit lifetime
    // We explicitly say this reference lives at least as long as 'a.
    y: &'a i32, 

    // 3. A mutable reference with an explicit lifetime
    // We can modify this value, and it also lives as long as 'a.
    z: &'a mut i32 
) {
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn main() {
    let a = 10;
    let b = 20;
    let mut c = 30;

    // We call the function. 
    // The compiler automatically figures out what 'a is based on the inputs.
    syntax_demo(&a, &b, &mut c);
}