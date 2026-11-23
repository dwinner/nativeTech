// 07_object_safety.rs

// A trait that is NOT object-safe because its method returns `Self`.
// "Self" refers to the specific concrete type (e.g., Tweet).
// When we use Trait Objects (dyn Cloneable), we forget the concrete type,
// so the compiler doesn't know how much memory to allocate for "Self".
pub trait Cloneable {
    fn clone_self(&self) -> Self;
}

#[derive(Debug)] 
pub struct Tweet {
    pub username: String,
    pub content: String,
}

// Implement the trait for Tweet
impl Cloneable for Tweet {
    fn clone_self(&self) -> Self {
        Tweet {
            username: self.username.clone(),
            content: self.content.clone(),
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: "rustacean".to_string(),
        content: "Object safety is important!".to_string(),
    };

    // 1. Valid Usage (Concrete Type)
    // This is fine, because the compiler knows specifically that `tweet` is a `Tweet`.
    // Therefore, it knows `clone_self` returns a `Tweet`.
    let tweet_clone = tweet.clone_self();
    println!("Cloned tweet (concrete): {:?}", tweet_clone);

    // 2. Invalid Usage (Trait Object)
    // The following lines demonstrate the Object Safety violation.
    // -----------------------------------------------------------------
    // INSTRUCTIONS: Uncomment the lines below to see the Compile Error.
    // -----------------------------------------------------------------
    
    // let cloneable_object: Box<dyn Cloneable> = Box::new(tweet);
    
    // println!("If this ran, we would have a trait object.");

    // -----------------------------------------------------------------
    // Explanation of the error you would see:
    // error[E0038]: the trait `Cloneable` cannot be made into an object
    //    ...
    //    the trait cannot be made into an object because method `clone_self`
    //    references the `Self` type in its return type.
    // -----------------------------------------------------------------
    
    println!("(Uncomment the code in 'main' to see the Object Safety error!)");
}