// TODO: Use two variants, one for a title error and one for a description error.
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   You'll have to update the implementation of `Ticket::new` as well.
/* TODO */
enum TicketNewError {
    /* TODO */,
}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
   /* TODO */}
}

#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub(crate) fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
           /* TODO */
          return Err("Title cannot be empty".to_string());;
        }
        if title.len() > 50 {
           /* TODO */
          return Err("Title cannot be longer than 50 bytes".to_string());;
        }
        if description.is_empty() {
           /* TODO */
          return Err("Description cannot be empty".to_string());;
        }
        if description.len() > 500 {
           /* TODO */
          return Err("Description cannot be longer than 500 bytes".to_string());;
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}
