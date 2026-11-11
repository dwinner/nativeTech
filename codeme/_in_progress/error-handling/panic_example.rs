fn check_critical_value(value: i32) {
    if value < 0 {
        // If the value is negative, it's an unrecoverable state for this
        // function.
        panic!("Critical value cannot be negative! Received: {}", value);
    }
    println!("Value {} is valid.", value);
}

fn main() {
    check_critical_value(10); // This will run fine.

    // The Line below will cause the program to panic and terminate.
    // check_critical_value(-5);

    // This line will not be reached if the panic occurs.
    println!("Program finished successfully.");
}