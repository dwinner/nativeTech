// 17_38_type_system.rs
fn find_user_score(user_id: &str) -> Option<u32> {
    match user_id {
        "Alice" => Some(100),
        "Bob" => Some(75),
        _ => None,
    }
}

fn string_to_positive_number(s: &str) -> Result<u32, String> {
    match s.parse::<i32>() {
        Ok(n) if n > 0 => Ok(n as u32),
        Ok(_) => Err(format!("'{}' is not positive", s)),
        Err(_) => Err(format!("Could not parse '{}'", s)),
    }
}

fn main() {
    // 1. Handling Option gracefully with combinators
    let user = "Alice";
    let message = find_user_score(user)
        .map(|score| format!("{} has {} points", user, score))
        .unwrap_or_else(|| format!("User {} unknown", user));
    
    println!("{}", message);

    // 2. Handling Result explicitly with match
    match string_to_positive_number("-5") {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}