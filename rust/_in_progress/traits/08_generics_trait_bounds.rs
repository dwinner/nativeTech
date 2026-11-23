// 08_generics_trait_bounds.rs

// We restrict T using "Trait Bounds":
// 1. PartialOrd: Allows us to use the '>' operator.
// 2. Copy: Allows us to move the value out of the slice (list[0]).
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // Works with integers
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // Works with chars
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}