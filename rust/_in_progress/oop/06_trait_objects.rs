// 06_trait_objects.rs

// 1. Define the Trait
pub trait Summarizable {
    fn summary(&self) -> String;
}

// 2. Define the structs
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

// 3. Implement the trait for the structs
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("New Rust Feature"),
        location: String::from("Blog"),
        author: String::from("Dev Team"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Learning Rust traits!"),
        reply: false,
        retweet: false,
    };

    // --- KEY CONCEPT: Trait Objects ---
    
    // We create a vector that holds "Trait Objects".
    // A standard Vec<T> can only hold one type (e.g., only Tweets).
    // A Vec<Box<dyn Summarizable>> can hold ANYTHING that implements Summarizable.
    
    let items_to_summarize: Vec<Box<dyn Summarizable>> = vec![
        // Box::new puts the data on the heap and gives us a pointer.
        // This coerces the specific type (Box<NewsArticle>) into the trait object (Box<dyn Summarizable>).
        Box::new(article), 
        Box::new(tweet),   
    ];

    // We can iterate and call methods via the trait object
    println!("--- Daily Feed ---");
    for item in items_to_summarize {
        // 'item' is of type Box<dyn Summarizable>
        // Rust figures out at RUNTIME which 'summary' method to call (Dynamic Dispatch).
        println!("Summary: {}", item.summary()); 
    }
}