// TODO: Implement the `Error` trait for `TicketNewError` using `thiserror`.
//   We've changed the enum variants to be more specific, thus removing the need for storing
//   a `String` field into each variant.
//   You'll also have to add `thiserror` as a dependency in the `Cargo.toml` file.

/* TODO */
pub enum TicketNewError {
    /* TODO */
    TitleCannotBeEmpty,
    /* TODO */
    TitleTooLong,
    /* TODO */
    DescriptionCannotBeEmpty,
    /* TODO */
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}
