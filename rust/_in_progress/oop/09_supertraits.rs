// 09_supertraits.rs

use std::fmt::Display; // Import the standard Display trait

// Define a trait that REQUIRES the implementing type to also implement Display.
// The syntax `trait Child: Parent` is called a "Supertrait".
// It means: "To implement PrintableSummary, you MUST also implement Display".
trait PrintableSummary: Display { 
    fn print_summary(&self);
}

struct Report {
    title: String,
    content: String,
}

// 1. First, we implement the "Parent" trait (Display).
// If we tried to skip this block, the compiler would complain when we tried 
// to implement PrintableSummary below.
impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Report: '{}'", self.title)
    }
}

// 2. NOW we can implement PrintableSummary
impl PrintableSummary for Report {
    fn print_summary(&self) {
        println!("--- Summary ---");
        // We can use "{}" (Display) on 'self' here because we KNOW
        // that anything implementing PrintableSummary MUST implement Display.
        println!("{}", self); 
        println!("Content length: {}", self.content.len());
        println!("---------------");
    }
}

fn main() {
    let report = Report {
        title: "Q1 Results".into(),
        content: "Sales were strong...".into(),
    };

    report.print_summary();
}