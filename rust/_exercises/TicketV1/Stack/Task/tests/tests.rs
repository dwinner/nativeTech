// TODO: based on what you learned in Stack lesson, replace `TODO` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), /* TODO */);
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), /* TODO */);
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), /* TODO */);
    }
}
