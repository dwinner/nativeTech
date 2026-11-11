// 10_tls_client.rs

// NOTE: This crate requires the 'pkg-config' package installed on your operating system
// to locate system libraries (like OpenSSL).
// Linux: sudo apt install pkg-config libssl-dev
// macOS: brew install pkg-config

// [dependencies]
// native-tls = "0.2"

use native_tls::{TlsConnector, TlsStream};
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain = "www.rust-lang.org";
    let port = 443; // HTTPS port
    let server_address = format!("{}:{}", domain, port);

    // 1. Create a TlsConnector builder and build the connector.
    // TlsConnector::new() provides default settings which usually work for
    // connecting to public servers by using the system's certificate trust store.
    let connector = TlsConnector::builder()
        .build()
        .map_err(|e| format!("Failed to build TLS connector: {}", e))?;

    println!("Attempting TCP connection to {}...", server_address);

    // 2. Establish a regular TCP connection to the server.
    let tcp_stream = TcpStream::connect(&server_address)
        .map_err(|e| format!("TCP connect to '{}' failed: {}", server_address, e))?;

    println!("TCP connection established.");
    println!("Attempting TLS handshake with domain '{}'...", domain);

    // 3. Wrap the TCP stream with the TlsConnector to perform the TLS handshake.
    // The `connect` method takes the domain name (for SNI and certificate validation)
    // and the existing TcpStream.
    let mut tls_stream: TlsStream<TcpStream> = connector.connect(domain, tcp_stream)
        .map_err(|e| format!("TLS handshake with '{}' failed: {}", domain, e))?;

    println!("TLS connection established successfully!");

    // 4. Now, the `tls_stream` can be used much like a regular TcpStream.
    // It implements the `Read` and `Write` traits. Data written to it will be
    // encrypted, and data read from it will be decrypted.
    let http_request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: RustBookClient/0.1\r\n\r\n",
        domain
    );

    println!("\nSending HTTP GET request...");

    tls_stream.write_all(http_request.as_bytes())
        .map_err(|e| format!("Failed to write HTTP request: {}", e))?;
    
    tls_stream.flush()
        .map_err(|e| format!("Failed to flush stream after request: {}", e))?;

    println!("HTTP GET request sent.");
    println!("\nReading HTTP response (first part):");

    let mut response_buffer = Vec::new();
    let mut temp_chunk = [0u8; 512]; // Read in chunks of 512 bytes

    // Read the server's response.
    match tls_stream.read(&mut temp_chunk) {
        Ok(0) => {
            println!("Server closed connection without sending a response.");
        }
        Ok(bytes_read) => {
            response_buffer.extend_from_slice(&temp_chunk[..bytes_read]);
            println!("Read {} bytes from server.", bytes_read);

            // Attempt to print the received part as UTF-8.
            match String::from_utf8(response_buffer) {
                Ok(response_str) => {
                    println!("--- Response Snippet (first {} bytes) ---", bytes_read);
                    println!("{}", response_str.trim_end());
                    if bytes_read == temp_chunk.len() {
                        println!("... (response might be longer)");
                    }
                    println!("--- End of Response Snippet ---");
                }
                Err(_) => {
                    println!("Response data is not valid UTF-8.");
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading HTTP response: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}