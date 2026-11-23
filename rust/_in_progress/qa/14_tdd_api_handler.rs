// 14_tdd_api_handler.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file:
// 1. Compile: rustc --test 14_tdd_api_handler.rs -o test_runner
// 2. Run:     ./test_runner
// ---------------------------------------------------------

// --- Production Code (The "Green" Implementation) ---

// Note: In a real project with Cargo, we would use:
// use serde::Serialize;
// #[derive(Serialize)]
pub struct User {
    id: u32,
    username: String,
}

/// A simple handler that simulates fetching a user.
/// Returns a JSON string on success, or an Error string on failure.
pub fn get_user_handler(id: u32) -> Result<String, String> {
    // In the "Red" phase, this function would have returned:
    // Err("Not implemented yet".to_string())

    // In the "Green" phase, we implement the minimal logic:
    match id {
        1 => {
            let user = User { id: 1, username: "Alice".to_string() };
            
            // NOTE: In a real project, use `serde_json::to_string(&user)`
            // Here, we manually format the JSON so this file runs without external dependencies.
            let json = format!(r#"{{"id":{},"username":"{}"}}"#, user.id, user.username);
            
            Ok(json)
        }
        _ => Err("User not found".to_string()),
    }
}

// --- The Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_success() {
        // 1. Define expectations
        let expected_json = r#"{"id":1,"username":"Alice"}"#;

        // 2. Execute the handler
        let result = get_user_handler(1);

        // 3. Verify success
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_json);
    }

    #[test]
    fn test_get_user_not_found() {
        // 1. Execute handler with invalid ID
        let result = get_user_handler(999); 

        // 2. Verify failure
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User not found");
    }
}

fn main() {
    println!("This file demonstrates TDD for an API Handler.");
    println!("Run with `rustc --test 14_tdd_api_handler.rs`");
}