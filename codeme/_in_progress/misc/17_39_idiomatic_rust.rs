// 17_39_idiomatic_rust.rs
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 🚫 Verbose / Manual
    // let mut result = Vec::new();
    // for &n in &numbers {
    //     if n % 2 == 0 { result.push(n * 2); }
    // }

    // ✅ Idiomatic / Concise
    let evens_doubled: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n % 2 == 0) // Select logic
        .map(|&n| n * 2)          // Transform logic
        .collect();               // Materialize result

    println!("Doubled evens: {:?}", evens_doubled);

    // ✅ Idiomatic String Formatting
    let name = "Rustacean";
    let age = 5;
    
    // Use format! instead of confusing string concatenation
    let greeting = format!("Hello, {}! You are {} years old.", name, age);
    println!("{}", greeting);
}