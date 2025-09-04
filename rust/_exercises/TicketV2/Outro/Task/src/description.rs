// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

/* TODO */
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn value(&self) -> &str {
        &self.0
    }
}

/* TODO */
