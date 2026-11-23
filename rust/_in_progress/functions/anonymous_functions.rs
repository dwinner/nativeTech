fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Use an anonymous function `|x| x * 2` with the map method.
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled); // Output: [2, 4, 6, 8, 10]

    // Use an anonymous function `|acc, &x| acc + x` with the fold method
    // to sum the values.
    // The first argument to fold (0) is the initial value for the
    // accumulator (`acc`).
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum of numbers: {}", sum); // Output: 15
}