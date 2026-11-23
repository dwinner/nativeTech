// 16_impl_blocks_with_lifetimes.rs

struct Highlight<'a> {
    part: &'a str,
}

// We must declare <'a> inside the impl angle brackets first.
// This tells the compiler: "I am about to use a lifetime named 'a in this block."
impl<'a> Highlight<'a> {
    
    fn announce(&self) {
        println!("Attention to: {}", self.part);
    }

    // Example of another method using the data
    fn len(&self) -> usize {
        self.part.len()
    }
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    
    let segment = Highlight {
        part: &text[4..19], // "quick brown fox"
    };

    // We can now call the method defined in the impl block
    segment.announce();
    println!("Length of segment: {}", segment.len());
}