fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("The number is one or two"),
        3 => println!("The number is three"),
        _ => println!("It's some other number"),
    }
}