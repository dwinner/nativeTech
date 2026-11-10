// 19_generic_parser_function.rs

// 1. The Trait (The Capability)
trait Parse<'a> {
    fn parse(input: &'a str) -> Self;
}

// 2. The Generic Function (The Abstraction)
// This function doesn't know what T is. It only knows that T implements Parse.
fn load_configuration<'a, T>(input: &'a str) -> T
where
    T: Parse<'a>,
{
    println!("Loading configuration...");
    // We delegate the work to the specific type's implementation
    T::parse(input)
}

// 3. A Concrete Type (for demonstration)
struct ServerConfig<'a> {
    port: &'a str,
}

// 4. Implementing the Trait
impl<'a> Parse<'a> for ServerConfig<'a> {
    fn parse(input: &'