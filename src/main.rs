use std::io::{stdout, stdin, Write};
use std::env;
use whoami;
mod input;
mod commands;

fn main(){
    loop {

        // Check for CLI flags
        commands::commands();

        let user = whoami::username(); // Current logged in user
        let host = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string()); // Device hostname
        let dir = env::current_dir().unwrap(); // Current directory

        // Print prompt with username, hostname and current directory
        // Flush output to ensure it prints before read_line
        print!("{}@{} {} > ", user, host, dir.display());
        let _ = stdout().flush();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Parse user input
        input::parse_input(&input);
    }
}