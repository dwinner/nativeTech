// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

struct WrappingU32 {
    value: u32,
}

/* TODO */

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

fn main() {}
