// 21_validating_references_impl.rs

// 1. The Trait Definition (Required context)
trait Parse<'a> {
    fn parse(input: &'a str) -> Self;
}

// 2. The Struct with References
// We declare <'a> to tell the compiler: 
// "address" and "port" are strictly tied to the lifetime 'a.
#[derive(Debug)]
struct ServerConfig<'a> {
    address: &'a str,
    port: &'a str,
}

// 3. The Implementation
// This line is often tricky for beginners. We have three 'a usages:
// 1. impl<'a>: We declare the lifetime generic for this block.
// 2. Parse<'a>: We are implementing the version of Parse tied to 'a.
// 3. ServerConfig<'a>: We are implementing it for the struct tied to 'a.
impl<'a> Parse<'a> for ServerConfig<'a> {
    fn parse(input: &'a str) -> Self {
        // A very simple parser that splits by space.
        // Note: split(' ') returns an iterator yielding slices of the original string.
        // No new memory is allocated for the address or port strings!
        let mut parts = input.split(' ');
        
        ServerConfig {
            address: parts.next().unwrap_or("localhost"),
            port: parts.next().unwrap_or("8080"),
        }
    }
}

fn main() {
    // The raw configuration data (owned string)
    let config_data = String::from("127.0.0.1 9000");

    // We parse it into our struct.
    // 'server' now borrows from 'config_data'.
    let server = ServerConfig::parse(&config_data);

    println!("Server Configuration:");
    println!("Address: {}", server.address);
    println!("Port:    {}", server.port);

    // If we dropped 'config_data' here, we could no longer use 'server'.
    // drop(config_data); // Uncommenting this would cause a compile error!
}