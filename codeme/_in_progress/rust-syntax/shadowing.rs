fn main() {
    let x = 5;
    let x = x + 1; // This shadows the previous x
    println!("The value of x is: {}", x);
}