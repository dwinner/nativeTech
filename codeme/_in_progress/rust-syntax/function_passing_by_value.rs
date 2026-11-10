fn takes_value_copy(mut some_integer: i32) {
    some_integer += 1;
    println!("Value inside function: {}", some_integer);
}

fn main() {
    let x = 5;
    takes_value_copy(x);
    println!("Original value of x after function call: {}", x); // x is still 5
}