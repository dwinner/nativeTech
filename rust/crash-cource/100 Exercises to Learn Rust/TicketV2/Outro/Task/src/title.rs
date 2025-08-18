// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

/* TODO */
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn value(&self) -> &str {
        &self.0
    }
}

/* TODO */}
