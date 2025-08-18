// TODO: based on what we just learned about ownership, it sounds like immutable references
//   are a good fit for our accessor methods.
//   Change the existing implementation of `Ticket`'s accessor methods take a reference
//   to `self` as an argument, rather than taking ownership of it.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

   // TODO:
    pub fn title(self) -> String {
        self.title
    }}

   // TODO:
    pub fn description(self) -> String {
        self.description
    }}

   // TODO:
    pub fn status(self) -> String {
        self.status
    }}
}
