// 1. Struct Definition and impl Block (from previous request)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Read-only method (&self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Read-only method (&self)
    fn has_valid_width(&self) -> bool {
        self.width > 0
    }

    // Modifying method (&mut self) - requires the instance to be 'mut'
    fn double_width(&mut self) {
        self.width *= 2;
    }
}

// 2. The main function (from current image)
fn main() {
    // Declare rect1 as mutable ('mut') because we will call a modifying method on it.
    let mut rect1 = Rectangle { width: 30, height: 50 };

    println!("--- Initial State ---");
    // Call read-only methods:
    println!("The area of rect1 is {}.", rect1.area());
    println!("Does rect1 have valid width? {}", rect1.has_valid_width());
    println!("Original width: {}", rect1.width);

    // Call the modifying method:
    rect1.double_width();
    println!("\n--- After calling double_width() ---");
    println!("Width after doubling: {}", rect1.width);
    
    // Call the area method again on the modified struct:
    println!("New area: {}", rect1.area());
}