// 17_32_complex_lifetime_errors.rs

fn main() {
    // --- The Pitfall: Lifetime Confusion ---
    // Lifetime errors often feel obscure because the compiler forces you to 
    // be explicit about how long returned references relate to input references.

    // The function below fails because Rust doesn't know if the returned &str 
    // should live as long as the input &String, or some other duration.
    /*
    fn get_part(data: &String) -> &str { // Error: missing lifetime specifier
        &data[0..5]
    }
    */

    // --- The Fix: Explicit Lifetime Annotation ---
    // We use <'a> to declare a lifetime.
    // We tell Rust: "The input `data` lives for 'a, and the returned reference 
    // will also live for 'a." 
    
    let my_data = String::from("Rust programming");
    let part = get_reference_to_part(&my_data);
    
    println!("Part of data: {}", part);
}

// Fixed signature with lifetime 'a
fn get_reference_to_part<'a>(data: &'a String) -> &'a str {
    // Basic safety check to avoid panic if string is short
    if data.len() >= 5 {
        &data[0..5]
    } else {
        &data[0..data.len()]
    }
}