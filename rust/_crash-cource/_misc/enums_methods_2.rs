enum Command {
    Start,
    Stop,
    Move(i32, i32),
}

fn process_command(command: Command) {
    match command {
        Command::Start => println!("Starting..."),
        Command::Stop => println!("Stopping..."),
        Command::Move(x, y) => println!("Moving to coordinates: x = {}, y = {}", x, y),
    }
}

fn main() {
    process_command(Command::Start);
    process_command(Command::Move(10, 20));
    process_command(Command::Stop);
}