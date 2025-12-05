use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::env;
use std::path::Path;

fn main() {
    loop {
        // Use '>' character as the prompt
        // Flush it to ensure it prints before read_line
        print!("> ");
        let _ = stdout().flush();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Split the input into the command and the arguments
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command { // Handle shell built-ins
            "cd" => { // cd builtin
                // Default to '/' if a directory was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) { // Set current directory  (or / if unspecified)
                    eprintln!("{}", e); // Throw error on failure
                }
            },
            command => { // Normal not built-in command
                // Run the command inserted by the user
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                // Wait until previous command has been executed before accepting another
                let _ = child.wait();
            }
        }
    }
}