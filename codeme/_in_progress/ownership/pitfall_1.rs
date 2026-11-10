fn main() {
    let pizza = String::from("Margherita");
    let lunch = pizza;

    // println!("{}", pizza); // • Error: pizza no longer owns the data!
    println!("{}", lunch); // • Works fine!
}