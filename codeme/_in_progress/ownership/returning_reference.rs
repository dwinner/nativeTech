fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Lifetime annotation <'a> ensures that the returned reference
    // lives at least as long as the shortest of the input references.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("Rust");
    let result; // Declare 'result' outside the inner scope
    {
        let string2 = String::from("Programming");
        result = longest(&string1, &string2);
        println!("The longest string is {}", result); // 'result' is valid
        // here because
        // it borrows either 'string1' or 'string2',
        // and both are still in scope.
    } // 'string2' goes out of scope and is dropped here.
    println!("The longest string is {}", result); // This is fine because
    // 'result' borrows from 'string1'
    // which is still valid.
    println!("String 1 is {}", string1); // 'string1' is still valid
}