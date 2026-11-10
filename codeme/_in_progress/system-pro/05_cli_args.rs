// 05_cli_args.rs
use std::env; // Required to use args()

fn main() {
    // env::args() returns an iterator over the command-line arguments.
    // We can collect these into a Vec<String>.
    let arguments: Vec<String> = env::args().collect();

    println!("Total arguments passed: {}", arguments.len());

    // Print each argument along with its index.
    // args[0] is typically the path used to execute the program.
    for (index, argument) in arguments.iter().enumerate() {
        println!("Argument [{}]: {}", index, argument);
    }

    // For our mini_grep, we expect at least two arguments after the program name.
    // So, arguments.len() should ideally be 3 or more (program name + query + file path).
    if arguments.len() < 3 {
        if arguments.len() > 0 { // Check if program name itself is available
            eprintln!("\nUsage: {} <query> <file_path>", arguments[0]);
        } else {
            eprintln!("\nUsage: <program_name> <query> <file_path>");
        }
        eprintln!("Error: Not enough arguments provided.");
        std::process::exit(1); // Exit with an error code
    }

    // If we reach here, we assume we have at least the query and file_path.
    // Note: arguments[0] is the program name.
    // So, arguments[1] would be the query, and arguments[2] the file_path.
    if arguments.len() >= 3 {
        let query_arg = &arguments[1];
        let file_path_arg = &arguments[2];

        println!("\nIntended query: {}", query_arg);
        println!("Intended file path: {}", file_path_arg);
    }
}