// 05_trait_objects.rs

pub trait Brew {
    fn extract(&self) -> String;
}

struct Moka {
    size: u8,
}
impl Brew for Moka {
    fn extract(&self) -> String {
        format!("Moka pot bubbling for {} cups", self.size)
    }
}

struct EspressoMachine {
    pressure: u8,
}
impl Brew for EspressoMachine {
    fn extract(&self) -> String {
        format!("Espresso machine pumping at {} bars", self.pressure)
    }
}

fn main() {
    // NOTE: We will cover Smart Pointers like 'Box' in detail in Chapter 10.
    // For now, just know that 'Box' allows us to store these different types
    // in the same list by putting them behind a pointer of a fixed size.
    
    // We create a Vector that holds "Trait Objects" (dyn Brew)
    let machines: Vec<Box<dyn Brew>> = vec![
        Box::new(Moka { size: 6 }),
        Box::new(EspressoMachine { pressure: 9 }),
    ];

    // We can iterate over them and call the trait method.
    // Rust figures out which specific .extract() method to call at runtime.
    for machine in machines {
        println!("Brewing: {}", machine.extract());
    }
}