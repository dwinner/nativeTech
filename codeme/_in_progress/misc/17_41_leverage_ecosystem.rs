// 17_41_leverage_ecosystem.rs

/* Add to Cargo.toml:
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
*/

use serde::{Deserialize, Serialize};

// 1. Derive macros generate the conversion code
#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    tags: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_json = r#"{
        "host": "localhost", 
        "port": 8080, 
        "tags": ["web", "api"]
    }"#;

    // 2. Deserialize: JSON String -> Rust Struct
    let config: ServerConfig = serde_json::from_str(raw_json)?;
    println!("Loaded: {:?}", config);

    // 3. Serialize: Rust Struct -> JSON String
    let output = serde_json::to_string_pretty(&config)?;
    println!("\nSerialized:\n{}", output);

    Ok(())
}