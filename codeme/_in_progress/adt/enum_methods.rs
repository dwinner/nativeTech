#[derive(Debug)] // Needed to print the Option<TrafficLight> values with {:?}
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // Associated function that safely converts a string into a TrafficLight,
    // returning Option<T> to handle possible failure.
    fn from_str(color: &str) -> Option<TrafficLight> {
        // Normalize the input and use a match expression for safety and exhaustiveness.
        match color.to_lowercase().as_str() {
            "red" => Some(TrafficLight::Red),
            "yellow" => Some(TrafficLight::Yellow),
            "green" => Some(TrafficLight::Green),
            _ => None, // Any other string results in None
        }
    }
}

fn main() {
    let green_light = TrafficLight::from_str("green");
    let invalid_light = TrafficLight::from_str("purple");

    println!("--- Option Results ---");
    println!("'green' -> {:?}", green_light);
    println!("'purple' -> {:?}", invalid_light);

    println!("\n--- Using 'if let' ---");
    // The `if let` statement checks if the result is Some(light)
    if let Some(light) = TrafficLight::from_str("Red") {
        // If it is Some, the block runs, and 'light' is bound to the inner value (TrafficLight::Red).
        println!("Successfully created from 'Red': {:?}", light);
    } else {
        // The optional 'else' block runs only if the result was None.
        println!("Could not create light from 'Red'.");
    }
    
    // Example of 'if let' running the else block
    if let Some(light) = TrafficLight::from_str("orange") {
        println!("Successfully created from 'orange': {:?}", light);
    } else {
        println!("Could not create light from 'orange'.");
    }
}