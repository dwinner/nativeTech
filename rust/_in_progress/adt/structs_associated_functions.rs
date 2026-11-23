// To print our struct with `{:?}`, we need to derive the Debug trait.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 1. Associated Function (Constructor)
    // Does not take &self or &mut self. Called using `Rectangle::new(...)`
    fn new(width: u32, height: u32) -> Self {
        // 'Self' is an alias for 'Rectangle'
        Rectangle { width, height }
    }

    // 2. Associated Function (Constructor)
    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }

    // 3. Method
    // Takes &self. Called using `instance.area()`
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Call associated functions using the StructName::function_name() syntax.
    let rect_from_new = Rectangle::new(10, 20);
    let square_rect = Rectangle::square(15);

    println!("--- Associated Function Calls ---");
    
    // We use `{:?}` (Debug format) to print the struct instance.
    println!("Rectangle from new: {:?}", rect_from_new); 
    
    // Call the method (`area`) on the instance created by the associated function.
    println!("Area of square: {}", square_rect.area());
}