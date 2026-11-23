// 02_trait_summarizable.rs

// Define a trait for things that can provide a summary
pub trait Summarizable {
    fn summary(&self) -> String; // Method signature: takes &self, returns String

    // Traits can also have default method implementations
    fn default_summary(&self) -> String {
        String::from("(Read more...)") // Default implementation
    }
}

// Define some structs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement the trait for NewsArticle
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // We don't need to implement default_summary, we can use the default
}

// Implement the trait for Tweet
impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // We can override the default implementation if we want
    fn default_summary(&self) -> String {
        if self.retweet {
            format!("Retweeted: {}", self.summary())
        } else {
            self.summary() // Or just call the required method
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Traits are cool!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Rust 1.XX Released!"),
        location: String::from("Online"),
        author: String::from("The Rust Team"),
        content: String::from("A new version of Rust brings many improvements..."),
    };

    println!("Tweet summary: {}", tweet.summary());
    println!("Article summary: {}", article.summary());
    println!("Tweet default summary: {}", tweet.default_summary());
    println!("Article default summary: {}", article.default_summary());
}