fn main() {
    let mut note = String::from("Rust is fast");

    let r1 = &note; // Immutable borrow
    let r2 = &note; // Immutable borrow
    // let r3 = &mut note; // Error: cannot borrow as mutable
    // because it is also borrowed as immutable

    println!("Note: {}, {}", r1, r2);

    let r3 = &mut note; // Mutable borrow is allowed here because no
    // immutable borrows are active
    r3.push_str(" and safe");
    println!("Updated note: {}", r3);
}