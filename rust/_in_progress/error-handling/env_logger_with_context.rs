use anyhow::{Context, Result};
use log::{info, error};
use std::fs;
use env_logger; // Needed to initialize the logger
use std::env; // Used to set a default log filter

// --- DEPENDENCY NOTE ---
// To use this code, you must add the following dependencies to your Cargo.toml:
// [dependencies]
// anyhow = "1.0"
// log = "0.4"
// env_logger = "0.9" # Or your preferred logger
// -----------------------

// Function that attempts to read a config file and adds context upon failure.
fn read_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .context(format!("Unable to read configuration file from '{}'", path))
}

fn main() -> Result<()> {
    // Initialize the logger. Default to INFO level if RUST_LOG is not set.
    env_logger::Builder::from_env(env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string()))
        .init();

    // Attempt to read a non-existent file to trigger the error
    match read_config("config.toml") {
        Ok(config) => {
            log::info!("Configuration read: {} bytes", config.len());
        }
        Err(e) => {
            // Log the error using anyhow's detailed formatting: {:?}
            // This prints the full error chain, including the context we added
            // and the original source error (e.g., "No such file or directory").
            error!("Failed to read configuration: {:?}", e);
        }
    }

    Ok(())
}