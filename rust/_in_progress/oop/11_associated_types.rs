// 11_associated_types.rs

// Simplified Iterator-like trait
pub trait SimpleIterator {
    // 'Item' is an ASSOCIATED TYPE. 
    // This acts as a placeholder that the implementor must define.
    type Item;

    // next() returns an Option of the associated Item type
    fn next(&mut self) -> Option<Self::Item>;
}

// Implement the trait for a simple counter
struct Counter {
    current: u32,
    max: u32,
}

impl SimpleIterator for Counter {
    // We specify that for Counter, the 'Item' type is specifically u32.
    // This ties 'SimpleIterator' to 'u32' for this specific struct.
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> { // Returns Option<u32>
        if self.current < self.max {
            let val = self.current;
            self.current += 1;
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter { current: 0, max: 3 };

    // We can call next() and get Option<u32>
    println!("{:?}", counter.next()); // Some(0)
    println!("{:?}", counter.next()); // Some(1)
    println!("{:?}", counter.next()); // Some(2)
    println!("{:?}", counter.next()); // None
}