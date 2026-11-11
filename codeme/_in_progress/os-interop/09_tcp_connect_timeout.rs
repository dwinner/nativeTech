// 09_tcp_connect_timeout.rs
use std::net::TcpStream;
use std::io::{self, ErrorKind};
use std::time::Duration;

/// Attempts to connect to a server and returns a custom, more descriptive error.
fn connect_to_server(server_addr: &str) -> Result<TcpStream, String> {
    // Set a timeout for the connection attempt itself.
    let timeout = Duration::from_secs(5);
    println!("Attempting to connect to {} (timeout: {:?})...", server_addr, timeout);

    // parse() converts the string "IP:PORT" into a SocketAddr which connect_timeout requires
    match TcpStream::connect_timeout(&server_addr.parse().unwrap(), timeout) {
        Ok(stream) => {
            println!("Connection successful!");
            Ok(stream)
        }
        Err(e) => {
            // Match on the error kind to provide a better error message.
            let error_message = match e.kind() {
                ErrorKind::ConnectionRefused => {
                    "Connection refused. Is the server running on that port?".to_string()
                }
                ErrorKind::TimedOut => {
                    "Connection timed out. Check network or firewall.".to_string()
                }
                _ => {
                    format!("An unexpected error occurred: {}", e)
                }
            };
            Err(error_message)
        }
    }
}

fn main() {
    // This address is unlikely to have a server running, so it should fail.
    let bad_address = "127.0.0.1:9999";
    println!("--- Testing connection to a non-existent server ---");

    if let Err(e) = connect_to_server(bad_address) {
        eprintln!("Operation failed as expected: {}", e);
    }

    // To test a success case, you would run one of the echo servers from
    // a previous example (e.g., on port 8080) and call:
    // connect_to_server("127.0.0.1:8080");
}