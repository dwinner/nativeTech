struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 0, y: 10 };

    match point {
        // This arm matches only if x is 0 AND the guard `if y < 5` is
        // true.
        Point { x: 0, y } if y < 5 => {
            println!("On the y-axis, but close to the origin (y < 5).");
        }
        // This arm matches for any other point where x is 0.
        Point { x: 0, y } => {
            println!("On the y-axis at y = {}", y);
        }
        // This arm matches only if y is 0 AND the guard `if x > 5` is
        // true.
        Point { x, y: 0 } if x > 5 => {
            println!("On the x-axis, far from the origin (x > 5).");
        }
        // This arm matches for any other point where y is 0.
        Point { x, y: 0 } => {
            println!("On the x-axis at x = {}", x);
        }
        // This arm matches any other point.
        Point { x, y } => {
            println!("Point is at ({}, {})", x, y);
        }
    }
}