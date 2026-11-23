use anyhow::{Result, bail};
use log::{info, warn, error, debug, trace}; // LevelFilter removed
use env_logger::Builder;
use std::env;

// --- DEPENDENCY NOTE ---
// To use this code, you must add the following dependencies to your Cargo.toml:
// [dependencies]
// log = "0.4"
// env_logger = "0.9"
// anyhow = "1.0"
// -----------------------

fn main() -> Result<()> {
    // Initialize env_logger. 
    Builder::from_env(env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string()))
        .init(); 

    info!("Application started.");

    // --- Test 1: Successful operation ---
    match risky_operation(10) {
        Ok(_) => info!("Operation successful."),
        Err(e) => {
            error!("Operation failed: {:?}", e);
        }
    }

    // --- Test 2: Operation failed due to negative input ---
    match risky_operation(-5) {
        Ok(_) => info!("Operation (negative) successful."),
        Err(e) => {
            // This matches your successful log output: [2025-11-09T20:30:04Z ERROR codetest] Operation (negative) failed: Value cannot be negative: -5
            error!("Operation (negative) failed: {:?}", e); 
        }
    }

    debug!("This is detailed debug information."); 
    trace!("This message is very verbose."); 

    info!("Application finished.");

    Ok(())
}

fn risky_operation(value: i32) -> Result<()> {
    if value < 0 {
        warn!("Attempting operation on negative value: {}", value);
        bail!("Value cannot be negative: {}", value);
    }
    
    debug!("Performing operation on value: {}", value); 
    
    Ok(())
}