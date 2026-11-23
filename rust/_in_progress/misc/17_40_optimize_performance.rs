// 17_40_optimize_performance.rs
fn create_inefficient(size: usize) -> Vec<usize> {
    let mut v = Vec::new(); // Capacity starts at 0
    for i in 0..size {
        // Triggers multiple reallocations and copies as 'v' grows
        v.push(i); 
    }
    v
}

fn create_efficient(size: usize) -> Vec<usize> {
    // Allocates the exact amount of memory needed upfront
    let mut v = Vec::with_capacity(size); 
    for i in 0..size {
        v.push(i); // No resizing needed
    }
    v
}

fn main() {
    let size = 1_000_000;

    // 1. Inefficient approach
    let _v1 = create_inefficient(size);
    println!("Inefficient vec created.");

    // 2. Efficient approach
    let _v2 = create_efficient(size);
    println!("Efficient vec created.");
}