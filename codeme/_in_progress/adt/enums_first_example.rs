// Define an enum for different kinds of web events
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

// A function to process different web events
fn inspect(event: WebEvent) {
    // The match expression must cover all variants of WebEvent.
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        
        // Destructures the char value into the variable 'c'
        WebEvent::KeyPress(c) => println!("Key pressed: '{}'.", c),
        
        // Destructures the i64 values into the variables 'x' and 'y'
        WebEvent::Click { x, y } => println!("Clicked at coordinates: x={}, y={}.", x, y),
    }
}

fn main() {
    // Instantiate each variant
    let load_event = WebEvent::PageLoad;
    let click_event = WebEvent::Click { x: 20, y: 80 };
    let key_event = WebEvent::KeyPress('x');

    println!("--- Inspecting Events ---");
    inspect(load_event);
    inspect(click_event);
    inspect(key_event);
}