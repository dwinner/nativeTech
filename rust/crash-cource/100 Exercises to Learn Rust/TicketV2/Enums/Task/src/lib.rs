// TODO: use `Status` as type for `Ticket::status`
//   Adjust the signature and implementation of all other methods as necessary.

#[derive(Debug, PartialEq)]
// `derive`s are recursive: it can only derive `PartialEq` if all fields also implement `PartialEq`.
// Same holds for `Debug`. Do what you must with `Status` to make this work.
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

/* TODO */
pub enum Status {
    /* TODO */

impl Ticket {
    pub fn new(title: String, description: String, status: St/* TODO */-> Ticket {
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

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &/* TODO */{
        &self.status
    }
}
