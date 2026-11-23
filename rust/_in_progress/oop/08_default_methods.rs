// 08_default_methods.rs

// Define the trait with one required method and one default method
trait Clickable {
    fn click(&self); // Required method (no body)

    // Method with a default implementation
    // Implementors (like Button) can use this as-is, or override it.
    fn hover_text(&self) -> String {
        String::from("Click me!") // Default text
    }
}

struct Button {
    label: String,
}

impl Clickable for Button {
    fn click(&self) {
        println!("Button '{}' clicked!", self.label);
    }
    // We do NOT implement hover_text here.
    // Button will automatically use the default implementation from the trait.
}

struct ImageLink {
    src: String,
}

impl Clickable for ImageLink {
    fn click(&self) {
        println!("Navigating to image link '{}'...", self.src);
    }

    // Overrides the default hover_text()
    // We define our own version, replacing the default logic.
    fn hover_text(&self) -> String {
        format!("View image: {}", self.src)
    }
}

fn main() {
    let button = Button { label: "Submit".into() };
    let image = ImageLink { src: "logo.png".into() };

    // 1. Button uses the default behavior
    button.click();
    println!("Button hover: {}", button.hover_text()); // Output: Click me!

    println!("---");

    // 2. ImageLink uses the overridden behavior
    image.click();
    println!("Image hover: {}", image.hover_text()); // Output: View image: logo.png
}