// 08_deref_coercion.rs
use std::rc::Rc;

// A simple struct to demonstrate method calls.
struct MyValue {
    data: i32,
}

impl MyValue {
    fn display(&self) {
        println!("MyValue data: {}", self.data);
    }
}

// A helper function that expects a string slice (&str).
// We will use this to prove that a Box<String> can be passed here directly.
fn print_str_slice(s: &str) {
    println!("Slice: {}", s);
}

fn main() {
    // 1. Setup Smart Pointers
    let b = Box::new(MyValue { data: 42 });
    let rc_val = Rc::new(MyValue { data: 100 });

    println!("--- Method Calls via Deref Coercion ---");

    // Calling display() on Box<MyValue>
    // Behind the scenes, Rust sees that Box implements Deref<Target=MyValue>.
    // It automatically turns &Box<MyValue> into &MyValue to match the method signature.
    b.display(); 

    // Similarly for Rc<MyValue>
    rc_val.display(); 

    println!("\n--- Explicit Dereference ---");
    // Explicit dereference is also possible using '*', but rarely needed for methods.
    (*b).display();
    (*rc_val).display();

    println!("\n--- Chained Coercion (Box<String> -> &str) ---");
    // Create a String inside a Box.
    let name_box = Box::new("Rustacean".to_string());

    // String -> &str coercion is standard in Rust.
    // Box<String> -> String -> &str is "Chained Coercion".
    // 1. Box derefs to String.
    // 2. String derefs to str.
    
    // We can call .is_empty() (a method on &str) directly on the Box!
    println!("Is '{}' empty? {}", name_box, name_box.is_empty()); 

    // We can pass &Box<String> to a function expecting &str.
    // Rust performs the conversions automatically.
    print_str_slice(&name_box); 
}