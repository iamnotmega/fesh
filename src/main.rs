use std::process::Command;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        // Use '>' character as the prompt
        // Flush it to ensure it prints before read_line
        print!("> ");
        let _ = stdout().flush();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Remove trailing newline from read_line
        let command = input.trim();

        // Run the command inserted by the user
        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // Wait until previous command has been executed before accepting another
        let _ = child.wait();
    }
}