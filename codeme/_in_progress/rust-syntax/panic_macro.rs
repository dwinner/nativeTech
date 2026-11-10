fn main() {
    let result = divide(10.0, 0.0);
    if let Err(e) = result {
        panic!("Application error: {}", e);
    }
}

fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}