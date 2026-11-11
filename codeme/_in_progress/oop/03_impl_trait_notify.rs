// 03_impl_trait_notify.rs

// --- Dependencies from previous example ---

pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// --- New Code Logic ---

// This function accepts ANY type that implements Summarizable
// This is known as "Polymorphism"
pub fn notify(item: &impl Summarizable) { // Using 'impl Trait' syntax
    println!("Breaking news! {}", item.summary());
}

fn main() {
    let tweet = Tweet { 
        username: String::from("rustacean"), 
        content: String::from("Traits are cool!"), 
        reply: false