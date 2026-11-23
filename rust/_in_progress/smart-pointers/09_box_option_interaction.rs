// 09_box_option_interaction.rs

#[derive(Debug)]
struct ImportantData {
    id: u32,
    payload: String,
}

// This struct owns an ImportantData instance on the heap.
struct DataOwner {
    name: String,
    // Box<T> is good when the struct needs to own T and T is large,
    // or T's size is unknown at compile time.
    // Option allows the data to be uninitialized (None) initially.
    data_ptr: Option<Box<ImportantData>>,
}

impl DataOwner {
    fn new(name: &str) -> Self {
        DataOwner {
            name: name.to_string(),
            data_ptr: None,
        }
    }

    fn initialize_data(&mut self, id: u32, payload: &str) {
        // We allocate memory on the heap using Box::new
        self.data_ptr = Some(Box::new(ImportantData {
            id,
            payload: payload.to_string(),
        }));
        println!("{} initialized data.", self.name);
    }

    fn update_payload(&mut self, new_payload: &str) {
        // self.data_ptr is an Option<Box<ImportantData>>
        
        // 1. .as_mut() gives us Option<&mut Box<ImportantData>>.
        //    This allows us to look at the Box without removing it from the struct.
        if let Some(boxed_data_mut_ref) = self.data_ptr.as_mut() {
            // 2. boxed_data_mut_ref is &mut Box<ImportantData>.
            //    We can assign to fields of ImportantData thanks to `DerefMut` on Box.
            //    Rust follows the pointer to the heap and updates the string there.
            boxed_data_mut_ref.payload = new_payload.to_string();
            
            println!(
                "{} updated payload for ID {}.", 
                self.name, 
                boxed_data_mut_ref.id
            );
        } else {
            println!("{} has no data to update.", self.name);
        }
    }

    fn display_data(&self) {
        // .as_ref() gives us Option<&Box<ImportantData>>
        if let Some(boxed_data_ref) = self.data_ptr.as_ref() {
            // boxed_data_ref is &Box<ImportantData>
            // We can access fields of ImportantData thanks to `Deref` on Box.
            println!(
                "{}'s Data [ID: {}]: {}", 
                self.name, 
                boxed_data_ref.id, 
                boxed_data_ref.payload
            );
        } else {
            println!("{} has no data to display.", self.name);
        }
    }
}

fn main() {
    let mut owner1 = DataOwner::new("OwnerAlpha");
    
    // 1. Initially empty
    owner1.display_data(); 

    // 2. Allocate on heap
    owner1.initialize_data(101, "Initial crucial data");
    owner1.display_data();

    // 3. Mutate data on heap via the struct
    owner1.update_payload("Updated important information");
    owner1.display_data();
}