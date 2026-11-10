// 09_closure_trait_bounds.rs

// This function accepts closures that only need immutable access (`Fn`).
fn call_reporter<F>(reporter: F)
where
    // Trait bound: F must implement Fn (immutable access)
    F: Fn() -> String, 
{
    println!("Report: {}", reporter());
}

// This function accepts closures that might mutate their environment (`FnMut`).
// The closure argument itself must be marked `mut` because FnMut implies modifying the closure's internal state.
fn call_mutator<F>(mut mutator: F) 
where
    // Trait bound: F must implement FnMut (mutable access)
    F: FnMut(), 
{
    // We can call it multiple times.
    mutator();
    mutator();
}

// This function accepts any closure but consumes it (`FnOnce`).
fn call_once<F>(consumer: F)
where
    // Trait bound: F must implement FnOnce (consumes captured variables/called only once)
    F: FnOnce(), 
{
    consumer();
    // Calling `consumer()` again here would cause a compile error because the closure is consumed.
}

fn main() {
    // --- 1. Fn Example ---
    let message = String::from("System status OK");
    // Captures `message` by immutable reference, so it implements `Fn`.
    let report_closure = || message.clone(); 
    call_reporter(report_closure);
    println!("Message is still valid: {}", message); // message is still usable

    println!("---");
    
    // --- 2. FnMut Example ---
    let mut counter = 0;
    // Captures `counter` by mutable reference, so it implements `FnMut`.
    let increment_closure = || {
        counter += 1;
        println!("Counter is now: {}", counter);
    };
    // We pass ownership of the closure to `call_mutator`.
    call_mutator(increment_closure);
    println!("Final counter value: {}", counter); // counter is modified

    println!("---");
    
    // --- 3. FnOnce Example ---
    let data = String::from("Consume me");
    // The `move` keyword forces this closure to take ownership of `data`, so it implements `FnOnce`.
    let consume_closure = move || {
        println!("Consumed: {}", data);
    };
    // The closure is consumed by `call_once`.
    call_once(consume_closure);
    // data is no longer valid here.
}