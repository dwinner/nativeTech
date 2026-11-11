fn main() {
    let width = 10;
    let height = 5;
    let area = calculate_area(width, height);

    println!("The area of the rectangle is: {}", area);
}

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}