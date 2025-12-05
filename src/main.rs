use std::process::Command;
use std::io::stdin;

fn main() {
    // Read user input
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // Remove trailing newline from read_line()
    let command = input.trim();

    // Run the command inserted by the user
    Command::new(command)
        .spawn()
        .unwrap();
}