// 07_generics_syntax_error.rs

fn main() {
    println!("This example demonstrates a compiler error.");
    println!("Uncomment the code below to see why Generics need Trait Bounds.");
}

// ---------------------------------------------------------------------------
// UNCOMMENT THE FUNCTION BELOW TO SEE THE ERROR
// ---------------------------------------------------------------------------

/*
// We read this as: "largest is a function generic over some type T"
fn largest<T>(list: &[T]) -> T {
    // Error 1: We cannot move out of a slice unless T implements 'Copy'
    let mut largest = list[0];

    for &item in list {
        // Error 2: We cannot compare 'item > largest' unless T implements 'PartialOrd'
        // NOTE: This specific line will cause a compiler error right now!
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/