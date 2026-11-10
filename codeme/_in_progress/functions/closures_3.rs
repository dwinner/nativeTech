use std::thread;

fn main() {
    let message = String::from("Data for the new thread");

    // `thread::spawn` requires a closure that can live for the entire
    // program ('static).
    // If we didn't use `move`, the closure would try to borrow `message`,
    // but the compiler
    // can't prove that `message` will live as long as the new thread.
    let handle = thread::spawn(move || {
        // The `move` keyword forces the closure to take ownership of
        // `message`.
        println!("Thread received: {}", message);
    });

    // `message` is no longer valid in the main thread.
    handle.join().unwrap();
}