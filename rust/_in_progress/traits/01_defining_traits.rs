// 1. Definition: The Trait
// This acts as the "contract". Any type implementing Brew *must*
// provide its own logic for the `extract` function.
pub trait Brew {
    fn extract(&self) -> String;
}

// 2. The Data: A Concrete Struct
struct Moka {
    cups: u8,
}

// 3. The Implementation: Signing the Contract
// We define specifically how a Moka pot fulfills the 'Brew' behavior.
impl Brew for Moka {
    fn extract(&self) -> String {
        format!("Bubbling on the stove... brewing {} cups of coffee.", self.cups)
    }
}

fn main() {
    let morning_coffee = Moka { cups: 6 };

    // We can call .extract() because Moka implements Brew
    println!("{}", morning_coffee.extract());
}