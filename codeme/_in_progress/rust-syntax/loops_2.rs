fn main() {
    for number in 1..=10 {
        // If the number is odd, skip the println! and go to the next
        // iteration.
        if number % 2 != 0 {
            continue;
        }
        // This line only runs for even numbers.
        println!("Found an even number: {}", number);
    }
}