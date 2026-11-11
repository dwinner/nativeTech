fn main() {
    let z = 5; // Immutable variable
    let z = z + 1; // Shadowing the previous variable
    println!("The value of z after shadowing is: {}", z);

    let z = "six"; // Shadowing with a different type
    println!("The value of z after shadowing with a different type is: {}", z);
}