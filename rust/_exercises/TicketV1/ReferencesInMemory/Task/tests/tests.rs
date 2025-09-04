pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this lesson, replace TODO with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), /* TODO */);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), /* TODO */);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), /* TODO */);
    }
}
