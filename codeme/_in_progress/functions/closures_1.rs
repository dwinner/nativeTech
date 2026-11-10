fn main() {
    // The compiler infers that `x` is an i32 and the return type is i32.
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));

    // You can also add explicit type annotations for clarity.
    let multiply = |a: i32, b: i32| -> i32 {
        a * b
    };
    println!("3 * 4 = {}", multiply(3, 4));
}