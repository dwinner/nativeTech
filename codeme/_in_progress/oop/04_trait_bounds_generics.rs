// 04_trait_bounds_generics.rs

use std::fmt::Debug;

// --- Setup: Traits and Structs ---

pub trait Summarizable {
    fn summary(&self) -> String;
}

// We add #[derive(Debug)] here so these structs satisfy the 'Debug' bound later
#[derive(Debug)] 
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

#[derive(Debug)]
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

// --- Main Logic: Generics & Bounds ---

// Equivalent to the previous 'impl Trait' example, but using explicit Generic syntax.
// This is useful if you need to refer to type 'T' multiple times (e.g., in return types).
pub fn notify_generic<T: Summarizable>(item: &T) { 
    println!("Breaking news (generic)! {}", item.summary());
}

// Can also use the 'where' clause for more complex bounds.
// This keeps the function signature clean when you have many constraints.
// Here, T must implement BOTH 'Summarizable' AND 'Debug'.
pub fn notify_complex<T>(item1: &T, item2: &T)
where 
    T: Summarizable + Debug 
{
    // Because T implements Debug, we can use "{:?}"
    println!("Item 1 Summary: {} | Debug: {:?}", item1.summary(), item1);
    println!("Item 2 Summary: {} | Debug: {:?}", item2.summary(), item2);
}

fn main() {
    let tweet1 = Tweet {
        username: String::from("rustacean"),
        content: String::from("Generics allow reusable code."),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("ferris"),
        content: String::from("Where clauses make things readable."),
        reply: true,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Rust Generics Explained"),
        location: String::from("The Book"),
        author: String::from("Steve"),
        content: String::from("..."),
    };

    // 1. Call the simple generic function
    notify_generic(&tweet1);
    notify_generic(&article);

    println!("---");

    // 2. Call the complex function
    // Note: Since notify_complex takes (item1: &T, item2: &T), 
    // both arguments must be the SAME type (both Tweets or both Articles).
    notify_complex(&tweet1, &tweet2);
    
    // The following would fail because T cannot be both Tweet and NewsArticle at once:
    // notify_complex(&tweet1, &article); 
}