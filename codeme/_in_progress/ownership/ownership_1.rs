fn main() {
    // From the first image
    let name = String::from("Mario");
    // `name` is now the owner of the data "Mario"
    println!("{}", name); // Prints "Mario"

    // From the second image
    let pasta = String::from("Carbonara");
    let dinner = pasta; // ownership moves from `pasta` to `dinner`

    // println!("{}", pasta); // • Error! `pasta` no longer owns the
    // data
    println!("{}", dinner); // • "Carbonara"
}