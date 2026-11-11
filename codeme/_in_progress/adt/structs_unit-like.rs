// 1. Unit-Like Struct Definition
// Has no fields. Used primarily for implementing traits or for markers.
struct Marker;

// 2. Trait Definition
trait MyTrait {
    // Defines a method that any implementing type must provide.
    fn description(&self) -> &'static str;
}

// 3. Trait Implementation for the unit-like struct
impl MyTrait for Marker {
    fn description(&self) -> &'static str {
        "This is a marker instance."
    }
}

fn main() {
    // Create an instance of the unit-like struct.
    // Note the lack of curly braces or parentheses.
    let m = Marker; 
    
    // Call the method defined by the implemented trait.
    println!("{}", m.description()); 
}