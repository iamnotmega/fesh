use std::io::{stdout, stdin, Write};
mod input;
mod commands;

fn main(){
    loop {

        // Check for CLI flags
        commands::commands();

        // Use '>' character as the shell prompt
        // Flush it to ensure it prints before read_line
        print!("> ");
        let _ = stdout().flush();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Parse user input
        input::parse_input(&input);
    }
}