// TODO: whenever `title` and `description` are returned via their accessor methods, they
//   should be normalized—i.e. leading and trailing whitespace should be removed.
//   There is a method in Rust's standard library that can help with this, but you won't
//   find it in the documentation for `String`.
//   Can you figure out where it is defined and how to use it?

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

    pub fn title(&self) -> &str {
        s/* TODO */    }

    pub fn description(&self) -> &str {
        s/* TODO */    }
}
