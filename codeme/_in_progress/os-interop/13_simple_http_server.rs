// 13_simple_http_server.rs
use std::fs;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

/// Handles a single client connection, parses a simple GET request, and serves a file.
/// This function will block until it is finished with the client.
fn handle_http_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let client_addr = stream.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());
    println!("Handling connection from {}", client_addr);

    // Wrap the stream in a BufReader to read lines
    let mut reader = BufReader::new(&stream);

    // --- 1. Read the HTTP Request Line ---
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        eprintln!("Failed to read request line from {}", client_addr);
        return Ok(()); // Close connection on read error
    }

    println!("Request from {}: {}", client_addr, request_line.trim());

    // --- 2. Read (and ignore) HTTP Headers ---
    // A real HTTP request has headers after the request line,
    // ending in a blank line (e.g., "\r\n"). We must read them
    // to consume the full request, even if we don't use them.
    let mut header_line = String::new();
    loop {
        match reader.read_line(&mut header_line) {
            Ok(0) => { // Client disconnected prematurely
                eprintln!("Client disconnected during header read.");
                return Ok(());
            }
            Ok(_) => {
                // If the line is just "\r\n" or "\n", it's the end of the headers.
                if header_line.trim().is_empty() {
                    break; // End of headers, break the loop
                }
                // We're just ignoring the header line in this simple server.
            }
            Err(e) => {
                eprintln!("Error reading headers: {}", e);
                return Err(e);
            }
        }
        header_line.clear(); // Clear string for the next line
    }

    // At this point, we've consumed the headers.

    // --- 3. Very basic request routing ---
    let (status_line, filename) = if request_line.starts_with("GET / HTTP/1.1") || request_line.starts_with("GET /index.html HTTP/1.1") {
        ("HTTP/1.1 200 OK", "index.html")
    } else if request_line.starts_with("GET /about.html HTTP/1.1") {
        ("HTTP/1.1 200 OK", "about.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // --- 4. Read File Content and Send Response ---
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            // If the specific file isn't found, try to send the generic 404 page
            println!("File '{}' not found for {}. Sending 404.", filename, client_addr);
            let not_found_page = "404.html"; // Assume this one exists
            let generic_404_content = fs::read_to_string(not_found_page)
                .unwrap_or_else(|_| "<h1>404 Not Found</h1><p>The requested resource was not found.</p>".to_string());
            
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                generic_404_content.len(),
                generic_404_content
            );

            // Send 404 response
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Error sending 404 response to {}: {}", client_addr, e);
            }
            if let Err(e) = stream.flush() {
                eprintln!("Error flushing 404 response to {}: {}", client_addr, e);
            }
            return Ok(());
        }
    };

    // --- 5. Construct and send the successful HTTP response ---
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        file_contents.len(),
        file_contents
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Error sending response to {}: {}", client_addr, e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("Error flushing response to {}: {}", client_addr, e);
    }

    println!("Response sent to {}. Closing connection.", client_addr);
    Ok(())
}

fn main() -> std::io::Result<()> {
    // --- Setup: Prepare dummy HTML files for the server to serve ---
    fs::write("index.html", "<h1>Welcome!</h1><p>This is the main page.</p><p><a href=\"/about.html\">About Us</a></p>")?;
    fs::write("about.html", "<h1>About Us</h1><p>We are a Rust learning example!</p><p><a href=\"/\">Home</a></p>")?;
    fs::write("404.html", "<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p><p><a href=\"/\">Go Home</a></p>")?;

    let listener_address = "127.0.0.1:7878";
    let listener = TcpListener::bind(listener_address)?;

    println!("Simple HTTP Server listening on http://{}", listener_address);

    // Accept connections one at a time (sequentially)
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                println!("Main: Accepted new connection. Handling...");
                // Handle the connection directly in the main thread.
                // The loop will block here until this client is done.
                if let Err(e) = handle_http_connection(stream) {
                    eprintln!("Connection error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Main: Failed to accept connection: {}", e);
            }
        }
    }

    // --- Cleanup (in a real server, this part wouldn't be reached) ---
    // fs::remove_file("index.html")?;
    // fs::remove_file("about.html")?;
    // fs::remove_file("404.html")?;

    Ok(())
}