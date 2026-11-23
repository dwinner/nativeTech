// Calculates the Euclidean distance between two 3D points represented as f64 tuples.
fn distance(point1: (f64, f64, f64), point2: (f64, f64, f64)) -> f64 {
    // Destructure the input tuples to get named coordinates
    let (x1, y1, z1) = point1;
    let (x2, y2, z2) = point2;

    // Calculate the distance squared for each axis
    let dx_sq = (x2 - x1).powi(2);
    let dy_sq = (y2 - y1).powi(2);
    let dz_sq = (z2 - z1).powi(2);

    // Sum the squares and take the square root
    (dx_sq + dy_sq + dz_sq).sqrt()
}

fn main() {
    // Define the two 3D points
    let point1 = (0.0, 0.0, 0.0);
    let point2 = (3.0, 4.0, 0.0);

    // Call the function and print the result
    println!("The distance between points is: {}", distance(point1, point2));
}