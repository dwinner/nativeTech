fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter is now: {}", counter);
        if counter == 5 {
            break; // Exits the loop
        }
    }
    println!("Loop finished.");
}