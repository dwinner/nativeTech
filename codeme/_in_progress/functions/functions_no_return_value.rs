// This function implicitly returns the unit type ().
fn log_message_implicit(message: &str) {
    println!("[LOG] {}", message);
    // No return value, so () is returned implicitly.
}

// This function explicitly returns the unit type ().
// It is functionally identical to the one above.
fn log_message_explicit(message: &str) -> () {
    println!("[LOG] {}", message);
    // We could write `return ()`; here, but it's not necessary.
}

fn main() {
    log_message_implicit("System online.");
    log_message_explicit("User logged in.");

    // You can see the unit type in action if you assign the result to a
    // variable.
    let result = log_message_implicit("Task finished.");
    // The type of `result` is `()`.
    // Printing it with debug formatting will show "()".
    println!("The result of a function returning the unit type is: {:?}",
    result);
}