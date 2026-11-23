// 04_orphan_rule.rs
use std::fmt;

// 1. Define a Local Trait
pub trait Brew {
    fn extract(&self) -> String;
}

// 2. Define a Local Type
struct Moka {
    size: u8,
}

// SCENARIO 1: Local Trait on External Type -> ALLOWED
// We defined 'Brew' in this crate, so we are allowed to implement it
// on 'String', even though 'String' comes from the standard library.
impl Brew for String {
    fn extract(&self) -> String {
        format!("Pour-over coffee using generic beans: {}", self)
    }
}

// SCENARIO 2: External Trait on Local Type -> ALLOWED
// We defined 'Moka' in this crate, so we are allowed to implement
// the standard library trait 'Display' on it.
impl fmt::Display for Moka {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Moka pot (Size: {})", self.size)
    }
}

// SCENARIO 3: External Trait on External Type -> FORBIDDEN
// This violates the Orphan Rule because we define neither the trait nor the type.
// Both 'Display' and 'String' belong to the standard library.
/*
impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
*/

fn main() {
    // Testing Scenario 1
    let coffee_beans = String::from("Arabica");
    println!("{}", coffee_beans.extract());

    // Testing Scenario 2
    let my_pot = Moka { size: 3 };
    // This works because we implemented Display
    println!("My coffee maker is: {}", my_pot); 
}