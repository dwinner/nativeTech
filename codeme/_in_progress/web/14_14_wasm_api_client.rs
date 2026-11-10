// 14_14_wasm_api_client.rs
/*
// Dependencies required in wasm_client/Cargo.toml
[package]
name = "wasm_client"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
# reqwest needs the wasm-bindgen feature for web targets
reqwest = { version = "0.12", features = ["json", "wasm-bindgen"] }
web-sys = { version = "0.3", features = ["console"] }
*/

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// --- Wasm Models ---
// These structs must be duplicated or put in a shared crate
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize)]
pub struct NewTodo {
    pub title: String,
}

// A simple macro to log to the browser console
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Our backend API URL
const API_URL: &str = "http://localhost:8080/todos";

/// Fetches the current list of todos from the backend API.
/// Returns the JSON string as a `JsValue`.
#[wasm_bindgen]
pub async fn fetch_todos() -> Result<JsValue, JsValue> {
    log!("Wasm: Fetching todos from {}", API_URL);

    let client = reqwest::Client::new();
    
    let resp = client
        .get(API_URL)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?; 

    // Handle non-successful HTTP status
    if resp.status().is_client_error() || resp.status().is_server_error() {
        let error_text = format!("HTTP Error: Status {}", resp.status().as_u16());
        return Err(JsValue::from_str(&error_text));
    }

    // Get the raw JSON string
    let json_text = resp
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Convert the Rust String into a JavaScript String (JsValue)
    Ok(JsValue::from_str(&json_text))
}

/// Adds a new todo item by POSTing to the backend API.
#[wasm_bindgen]
pub async fn add_todo(title: String) -> Result<JsValue, JsValue> {
    log!("Wasm: Adding todo: {}", &title);
    
    let new_todo = NewTodo { title };
    let client = reqwest::Client::new();
    
    let resp = client
        .post(API_URL)
        .json(&new_todo) // Serializes NewTodo to JSON
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Handle non-successful HTTP status
    if resp.status().is_client_error() || resp.status().is_server_error() {
        let error_text = format!("HTTP Error: Status {}", resp.status().as_u16());
        return Err(JsValue::from_str(&error_text));
    }

    // Get the raw JSON string
    let json_text = resp
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Convert the Rust String into a JavaScript String (JsValue)
    Ok(JsValue::from_str(&json_text))
}