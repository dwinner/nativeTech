//01_averaged_collection.rs

// Define the struct in its own scope (like a module would provide)
pub mod math_utils { 
    // Using a module to demonstrate privacy
    
    #[derive(Debug)] // Allow printing the struct
    pub struct AveragedCollection {
        list: Vec<i32>, // Private: External code cannot directly modify the list
        average: f64,   // Private: External code cannot directly set the average
    }

    impl AveragedCollection {
        // Public constructor (associated function)
        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: Vec::new(),
                average: 0.0,
            }
        }

        // Public method to add an element, updating the average correctly
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average(); // Call private helper method
        }

        // Public method to remove an element (if present)
        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop(); // pop() returns Option<i32>
            match result {
                Some(val) => {
                    self.update_average(); // Update average if removal happened
                    Some(val)
                }
                None => None, // List was empty
            }
        }

        // Public method to get the current average (read-only access)
        pub fn average(&self) -> f64 {
            self.average
        }

        // Private helper method to recalculate the average
        // Not marked 'pub', so only usable within this module/impl block
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = if self.list.is_empty() {
                0.0
            } else {
                total as f64 / self.list.len() as f64
            };
        }
    }
} // end module math_utils

fn main() {
    // We need 'use' to bring the public items into scope
    use math_utils::AveragedCollection;

    let mut collection = AveragedCollection::new();

    // We can only interact via public methods
    collection.add(10);
    collection.add(20);
    collection.add(30);

    println!("Average after adds: {}", collection.average()); // Output: 20.0

    // The following lines would cause compile errors because the fields are private:
    // collection.list.push(100); 
    // collection.average = 50.0; 

    collection.remove();
    println!("Average after remove: {}", collection.average()); // Output: 15.0

    // The following line would cause a compile error because the method is private:
    // collection.update_average(); 

    println!("Final collection state: {:?}", collection);
}