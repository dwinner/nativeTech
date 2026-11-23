fn main() {
    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("The numbers are equal"),
        (x, y) if x + y == 0 => println!("The numbers are opposites"),
        (x, y) => println!("Different numbers: ({}, {})", x, y),
    }
}