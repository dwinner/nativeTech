use std::cmp::PartialEq;

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Ticket {
            title,
            description,
            status,
        }
    }
}

// TODO: Implement the `PartialEq` trait for `Ticket`.
impl PartialEq for Ticket {
    f/* TODO */}
