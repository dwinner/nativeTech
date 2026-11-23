// 06_clap_args.rs
//
// To run this, add `clap` to your Cargo.toml dependencies:
// [dependencies]
// clap = { version = "4.4", features = ["derive"] }

use clap::Parser; // Import the Parser trait

/// A simple program to greet a person, demonstrating clap.
#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "0.1.0", about = "Greets a person - clap example", long_about = None)]
struct CliArgs {
    /// The name of the person to greet
    #[arg(short, long)] // Allows -n <NAME> or --name <NAME>
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)] // Allows -c <COUNT> or --count <COUNT>, defaults to 1
    count: u8,

    /// Optional message to include in the greeting
    #[arg(long)] // Allows --message <MESSAGE>
    message: Option<String>,

    // For our mini_grep example, it might look more like:
    // query: String,
    // file_path: std::path::PathBuf,
    // #[arg(short, long, action = clap::ArgAction::SetTrue)] // for a flag like -i
    // ignore_case: bool,
}

fn main() {
    // CliArgs::parse() will parse arguments from std::env::args(),
    // handle errors, and provide help/version messages automatically.
    let args = CliArgs::parse();

    for _ in 0..args.count {
        if let Some(ref msg) = args.message {
            println!("Hello, {}! Here's your message: {}", args.name, msg);
        } else {
            println!("Hello, {}!", args.name);
        }
    }

    // If --help or --version was passed, clap handles it and exits before this point.
    // If parsing failed, clap prints an error and exits.

    // test it with: cargo run -- --name Alice --count 3 --message "Welcome to Rust!"
}