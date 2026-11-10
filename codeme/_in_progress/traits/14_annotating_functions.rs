// 14_annotating_functions.rs

// We declare generic lifetime 'a.
// We tell the compiler: "x, y, and the return value must all live at least as long as 'a".
// Practically, 'a becomes the intersection (the smaller) of the lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let string2 = "xyz";

    // We pass references to both strings.
    // The compiler sees that both live until the end of main, so the result is valid.
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    // DEMO: Nested scopes
    {
        let string3 = String::from("tiny");
        // Here, 'result2' is valid because 'string3' is still alive inside this block.
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("The longest inner string is {}", result2);
    }
}