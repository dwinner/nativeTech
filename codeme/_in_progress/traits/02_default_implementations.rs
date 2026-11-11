// 02_default_implementations.rs

pub trait Brew {
    // REQUIRED: This method has no body. 
    // Every type implementing this trait MUST define this behavior.
    fn extract(&self) -> String;

    // OPTIONAL: This method has a default body.
    // Types can use this as-is, or override it if they need custom logic.
    fn clean(&self) {
        println!("Running standard hot water rinse...");
    }
}

// Scenario 1: Using the default behavior
struct Moka {
    cups: u8,
}

impl Brew for Moka {
    fn extract(&self) -> String {
        format!("Brewing {} cups on the stove.", self.cups)
    }
    // We do NOT implement clean() here, so Moka uses the default logic.
}

// Scenario 2: Overriding the default behavior
struct EspressoMachine {
    pressure: u8,
}

impl Brew for EspressoMachine {
    fn extract(&self) -> String {
        format!("Extracting at {} bar pressure.", self.pressure)
    }

    // We override clean() because this machine needs a specific cleaning process.
    fn clean(&self) {
        println!("Running high-pressure backflush cycle...");
    }
}

fn main() {
    let moka = Moka { cups: 3 };
    let machine = EspressoMachine { pressure: 9 };

    println!("--- Moka (Default Clean) ---");
    println!("{}", moka.extract());
    moka.clean(); // Prints: "Running standard hot water rinse..."

    println!("\n--- Espresso Machine (Overridden Clean) ---");
    println!("{}", machine.extract());
    machine.clean(); // Prints: "Running high-pressure backflush cycle..."
}