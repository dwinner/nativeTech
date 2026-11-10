fn main() {
    let mut article = String::from("Rust is awesome");
    update_article(&mut article); // Borrow article mutably
    println!("Updated article: {}", article);
    // article is still valid here
}

fn update_article(s: &mut String) {
    s.push_str(" for system programming!");
}