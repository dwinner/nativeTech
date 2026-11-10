// 20_zero_copy_parser.rs

// 1. THE CAPABILITY (Trait)
// We define a contract for anything that can be parsed from a string.
// The lifetime 'a ensures we can hold references to the original input string (Zero-Copy).
trait Parse<'a> {
    fn parse(input: &'a str) -> Self;
}

// 2. THE ABSTRACTION (Generic Function)
// This function can load ANY configuration type T, as long as T implements Parse.
fn load_configuration<'a, T>(input: &'a str) -> T
where
    T: Parse<'a>,
{
    println!("System: Loading configuration from source...");
    T::parse(input)
}

// 3. THE DATA (Struct with Lifetimes)
// This struct does not own the strings! It holds references to lines in the original file.
// This is extremely efficient (no memory allocation for strings).
#[derive(Debug)]
struct ServerConfig<'a> {
    ip_address: &'a str,
    port: &'a str,
}

// 4. THE IMPLEMENTATION (Binding it all together)
impl<'a> Parse<'a> for ServerConfig<'a> {
    fn parse(input: &'a str) -> Self {
        // Simple parsing logic: Split by space
        // Input: "127.0.0.1 8080"
        let mut parts = input.split_whitespace();

        ServerConfig {
            ip_address: parts.next().unwrap_or("localhost"),
            port: parts.next().unwrap_or("80"),
        }
    }
}

fn main() {
    // The "File": A String literal with static lifetime (or any string)
    let config_file_contents = "192.168.1.50 3000";

    // CALLING THE GENERIC FUNCTION:
    // We tell Rust: "Load this string into a ServerConfig struct"
    let config: ServerConfig = load_configuration(config_file_contents);

    println!("Success! Server configured:");
    println!("  IP:   {}", config.ip_address);
    println!("  Port: {}", config.port);
}