// This function calculates a sum and a product and returns both in a
// tuple.
fn calculate_sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b) // The last expression in a function is its return
    // value
}

fn main() {
    let input1 = 10;
    let input2 = 5;

    // Call the function and destructure the returned tuple directly into
    // variables.
    let (sum_result, product_result) = calculate_sum_and_product(input1,
        input2);

    println!("For {} and {}:", input1, input2);
    println!("  Sum: {}", sum_result);     // Output: 15
    println!("  Product: {}", product_result); // Output: 50
}