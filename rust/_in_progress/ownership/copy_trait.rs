fn main() {
    // `x` is an i32, which implements the `Copy` trait.
    let x = 5;

    // Because `x`'s type is `Copy`, a bit-for-bit copy of the value 5
    // is made and assigned to `y`. `x` is not moved or invalidated.
    let y = x;

    println!("x = {}, y = {}", x, y); // Both x and y are valid and can be
    // used.

    // Let's look at a non-Copy type for contrast.
    let s1 = String::from("hello");
    // `String` does not implement `Copy`, so this is a move.
    let s2 = s1;

    // The line below would cause a compile-time error because s1 was
    // moved.
    // println!("s1 = {}, s2 = {}", s1, s2);
    // error[E0382]: borrow of moved value: `s1`
}