/// Calculates the area of a rectangle.
///
/// This function takes the width and height of a rectangle and returns
/// its area.
/// It demonstrates good naming conventions and clear documentation.
///
/// # Parameters
///
/// * `width`: A `u32` representing the width of the rectangle. Must be
///   positive.
/// * `height`: A `u32` representing the height of the rectangle. Must be
///   positive.
///
/// # Returns
///
/// A `u32` representing the calculated area of the rectangle.
///
/// # Panics
///
/// This function will panic if either `width` or `height` is zero, as a
/// rectangle with a zero dimension is considered invalid in this context.
pub fn calculate_rectangle_area(width: u32, height: u32) -> u32 {
    if width == 0 || height == 0 {
        panic!("Both width and height must be non-zero.");
    }
    width * height
}

fn main() {
    let area = calculate_rectangle_area(10, 20);
    println!("The calculated area is: {}", area);
}