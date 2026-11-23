fn main() {
    let mut title = String::from("Rust Basics");

    modify_title(&mut title); // Borrow title mutably
    println!("Modified title: {}", title); // title is still valid here
}

fn modify_title(s: &mut String) {
    s.push_str(" - Advanced Topics");
}