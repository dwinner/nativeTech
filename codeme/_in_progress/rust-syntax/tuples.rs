fn main() {
    // A tuple holding an integer, a float, and a character.
    let my_tuple: (i32, f64, char) = (500, 6.4, 'R');

    // Method 1: Destructuring with a `let` binding.
    // This is a form of pattern matching that breaks the tuple into
    // separate variables.
    let (x, y, z) = my_tuple;

    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    // Method 2: Direct access using dot notation and the element's index.
    // Indices start from 0.
    let first_element = my_tuple.0;
    let second_element = my_tuple.1;
    println!("Direct access: First element is {}, second is {}", first_element, second_element);
}