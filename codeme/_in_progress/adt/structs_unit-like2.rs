// A unit-like struct - it holds no data.
struct MyStruct; 

// A marker trait - it defines no methods, it only marks a type.
trait MyMarker {}

// Implementing the marker trait for the unit-like struct.
// MyStruct now has the 'MyMarker' capability.
impl MyMarker for MyStruct {}

// An example function that requires the MyMarker trait
fn needs_marker<T: MyMarker>(item: T) {
    println!("Item of type {} has the MyMarker trait.", std::any::type_name::<T>());
}

fn main() {
    // 1. Create an instance of the unit-like struct
    let instance = MyStruct; 

    // 2. We can now call a function that requires the trait
    needs_marker(instance);
    
    // We can't do much with 'instance' itself since it has no fields or methods, 
    // but its *type* is valuable for the compiler.
}