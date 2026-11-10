fn main() {
    // Tuple containing a Copy type (u32), a non-Copy type (String), and another Copy type (f64)
    let employee: (u32, String, f64) = (1001, String::from("John Doe"), 75000.0);

    println!("--- Accessing Tuple Fields ---");
    
    // 1. Copy: u32 is copied. Ownership of the original value remains in employee.
    let id = employee.0;
    
    // 2. Borrow: String is non-Copy. We take a reference (&) to the String.
    // This avoids moving ownership, preserving the validity of the 'employee' tuple.
    let name = &employee.1;
    
    // 3. Copy: f64 is copied.
    let salary = employee.2;

    println!("Employee ID: {}", id);
    println!("Employee Name: {}", name);
    println!("Employee Salary: ${}", salary);
    
    println!("\n--- Original Tuple is Still Valid ---");
    // Because we only copied the ID and salary, and borrowed the name, 
    // we can still print the entire tuple instance (requires Debug trait).
    // Note: We'd need #[derive(Debug)] on the tuple itself (which works automatically 
    // for standard types).
    println!("Tuple after access: ({}, {}, {})", employee.0, employee.1, employee.2);
}