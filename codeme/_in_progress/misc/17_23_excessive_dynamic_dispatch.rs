// 17_23_excessive_dynamic_dispatch.rs

trait Drawable {
    fn draw(&self);
}

struct Button { id: u32 }
impl Drawable for Button {
    fn draw(&self) { println!("Drawing Button {}", self.id); }
}

struct Label { text: String }
impl Drawable for Label {
    fn draw(&self) { println!("Drawing Label: {}", self.text); }
}

fn main() {
    let button = Button { id: 1 };
    let label = Label { text: "Submit".to_string() };

    println!("--- Static Dispatch (Performance Fix) ---");
    // The compiler generates specific machine code for `Button` and `Label`.
    // Calls are direct and can be inlined.
    render_static(&button); 
    render_static(&label);

    println!("\n--- Dynamic Dispatch (Flexibility/Pitfall) ---");
    // Essential for heterogeneous lists (mixed types), but has runtime overhead.
    // Usage involves looking up the function address in a "vtable".
    let elements: Vec<&dyn Drawable> = vec![&button, &label];
    
    for el in elements {
        render_dynamic(el); 
    }
}

// --- The Pitfall: Dynamic Dispatch ---
// Requires a pointer indirection (vtable lookup) at runtime.
// Compiler cannot inline these calls easily.
fn render_dynamic(element: &dyn Drawable) {
    element.draw(); 
}

// --- The Fix: Static Dispatch ---
// Uses generics. The compiler performs "Monomorphization", creating 
// a unique version of this function for every concrete type used.
fn render_static<T: Drawable>(element: &T) {
    element.draw(); 
}