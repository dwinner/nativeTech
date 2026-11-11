// 13_state_pattern_enum.rs

// Define the internal data for each specific state
struct DraftPost { content: String }
struct PendingReviewPost { content: String }
struct PublishedPost { content: String }

// The main object holding the current state as an Enum variant
enum PostState {
    Draft(DraftPost),
    PendingReview(PendingReviewPost),
    Published(PublishedPost),
}

pub struct Post {
    state: PostState,
}

impl Post {
    pub fn new() -> Post {
        Post { 
            state: PostState::Draft(DraftPost { content: String::new() }) 
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // Only allowed in Draft state
        if let PostState::Draft(ref mut draft) = self.state {
            draft.content.push_str(text);
        } else {
            println!("Cannot add text in current state.");
            // In a real app, might return Result<(), Error>
        }
    }

    pub fn request_review(&mut self) {
        // Transition from Draft to PendingReview
        // logic: We need to take ownership of the data inside 'self.state'.
        // However, we only have a mutable reference (&mut self).
        // std::mem::replace swaps the current state with a temporary placeholder, 
        // giving us ownership of the old state so we can move its content.
        
        let temp_state = std::mem::replace(&mut self.state, PostState::Draft(DraftPost{content: String::new()}));
        
        if let PostState::Draft(draft) = temp_state {
            self.state = PostState::PendingReview(PendingReviewPost { content: draft.content });
        } else {
            // If it wasn't a draft, put it back (or handle error). 
            // For this simple example, we just printed an error, but we technically lost the old state 
            // because of the swap. In production code, you'd handle this more gracefully.
            println!("Post must be in Draft state to request review.");
            self.state = temp_state; // Restore original state
        }
    }

    pub fn approve(&mut self) {
        // Transition from PendingReview to Published
        let temp_state = std::mem::replace(&mut self.state, PostState::Draft(DraftPost{content: String::new()}));

        if let PostState::PendingReview(pending) = temp_state {
            self.state = PostState::Published(PublishedPost { content: pending.content });
        } else {
            println!("Post must be Pending Review to approve.");
            self.state = temp_state; // Restore original state
        }
    }

    pub fn content(&self) -> &str {
        // Access content based on current state
        match &self.state {
            PostState::Draft(s) => &s.content,
            PostState::PendingReview(s) => &s.content,
            PostState::Published(s) => &s.content,
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("Learning about state patterns in Rust. ");
    println!("Content (Draft): {}", post.content());

    println!("--- Requesting Review ---");
    post.request_review();
    
    post.add_text("This won't be added."); // Tries adding text in wrong state

    println!("--- Approving ---");
    post.approve();
    
    println!("Content (Published): {}", post.content());

    println!("--- Invalid Transition Check ---");
    post.request_review(); // Tries invalid transition from Published -> Review
}