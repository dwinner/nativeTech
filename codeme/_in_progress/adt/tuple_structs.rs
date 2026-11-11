// Define tuple structs
// Color is a distinct type from (u8, u8, u8)
struct Color(u8, u8, u8); // Represents RGB components
// Point is a distinct type from (i32, i32)
struct Point(i32, i32); // Represents 2D coordinates

fn main() {
    // Instantiate like tuples, but with the type name
    let red = Color(255, 0, 0);
    let origin = Point(0, 0);

    println!("--- Accessing Fields ---");
    // Access fields using dot notation and index, just like a regular tuple.
    // Indices start at 0.
    println!("Red's red component: {}", red.0);
    println!("Red's green component: {}", red.1); 
    println!("Origin X coordinate: {}", origin.0);

    println!("\n--- Destructuring ---");
    // Tuple structs can be destructured using a pattern match.
    // The type name is required in the pattern.
    let Point(x, y) = origin;
    println!("Origin coordinates: x={}, y={}", x, y);

    println!("\n--- Type Distinction ---");
    // You cannot assign a tuple struct instance to a plain tuple type
    // without explicit conversion, because they are distinct types.
    // let point_tuple: (i32, i32) = origin; // This would cause a type error!
    
    // However, you can destructure them into regular primitives:
    let Color(r, g, b) = red;
    println!("Destructured Color: R={}, G={}, B={}", r, g, b);
}