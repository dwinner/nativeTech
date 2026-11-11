fn main() {
    for number in 1..5 {
        println!("Exclusive range value: {}", number);
    }

    for number in 1..=5 {
        println!("Inclusive range value: {}", number);
    }
}