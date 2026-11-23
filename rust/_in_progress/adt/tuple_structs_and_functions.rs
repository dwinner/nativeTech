// Define a tuple struct to represent 3D dimensions (Length, Width, Height)
// using f64 for floating-point values.
struct Dimensions(f64, f64, f64); 

// Function that takes an instance of the Dimensions tuple struct
// and calculates its volume.
fn calculate_volume(dimensions: Dimensions) -> f64 {
    // Access fields using dot notation and the zero-based index.
    dimensions.0 * dimensions.1 * dimensions.2
}

fn main() {
    // Instantiate the struct. The values are assigned to fields 0, 1, and 2, respectively.
    let package = Dimensions(30.5, 20.0, 15.0);

    // Print the individual dimensions using index access.
    println!("Package dimensions: Length: {}, Width: {}, Height: {}",
             package.0, package.1, package.2);
             
    // Pass the 'package' instance to the function.
    // Note: Since f64 and tuple structs composed of Copy types are Copy, 
    // the value is copied, not moved, to the function parameter.
    println!("Package volume: {} cubic units", calculate_volume(package));
}