// 03_implementing_on_custom_types.rs

// 1. The Trait Definition (The Interface)
pub trait Brew {
    fn extract(&self) -> String;
}

// 2. The Custom Type (The Data)
struct Moka {
    size: u8, // Capacity in cups
}

// 3. The Implementation (Binding Behavior to Data)
impl Brew for Moka {
    fn extract(&self) -> String {
        format!(
            "Bubbling on the stove... ready to serve {} cups of rich coffee!",
            self.size
        )
    }
}

fn main() {
    // Create an instance of our custom struct
    let my_moka = Moka { size: 6 };

    // Call the trait method.
    // The compiler knows which implementation to use based on the type (Moka).
    println!("{}", my_moka.extract());
}