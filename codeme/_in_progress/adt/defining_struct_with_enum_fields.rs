#[derive(Debug)] // For printing Status
enum Status {
    Online,
    Offline,
    Connecting { attempts: u32 }, // Status with data
    Maintenance,
}

#[derive(Debug)] // For printing Device
struct Device {
    id: u32,
    name: String,
    status: Status, // Enum used as a field
}

impl Device {
    fn new(id: u32, name: String) -> Device {
        // Chapter 5

        Device {
            id,
            name,
            status: Status::Offline, // Default status
        }
    }

    // Method to update the device status
    fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    // Method to get a descriptive status message
    fn get_status_message(&self) -> String {
        match &self.status {
            // Corrected: Two placeholders match self.name and self.id
            Status::Online => format!("Device '{}' ({}) is online.",
                                    self.name, self.id),
            // Corrected: Two placeholders match self.name and self.id
            Status::Offline => format!("Device '{}' ({}) is offline.",
                                    self.name, self.id),
            // This variant was likely correct as it used attempts:
            Status::Connecting { attempts } => format!("Device '{}' is connecting (attempt {}).",
                                                    self.name, attempts),
            // Corrected: Two placeholders match self.name and self.id
            Status::Maintenance => format!("Device '{}' ({}) is under maintenance.",
                                        self.name, self.id),
        }
    }
}

fn main() {
    let mut router = Device::new(101, String::from("Main Router"));

    println!("{}", router.get_status_message());

    router.set_status(Status::Connecting { attempts: 1 });
    println!("{}", router.get_status_message());

    router.set_status(Status::Online);
    println!("{}", router.get_status_message());

    // Debug print the whole struct
    println!("Current device state: {:?}", router); 
}