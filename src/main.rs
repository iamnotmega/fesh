use std::process::{Child, Stdio, Command};
use std::env;
use std::io::{stdout, stdin, Write};
use std::path::Path;

fn main(){

    // Check for --version flag to display shell version
    if std::env::args().any(|a| a == "--version" || a == "-v")  {
        println!("Fesh {} ({})", env!("CARGO_PKG_VERSION"), std::env::consts::ARCH);
        return;
    }

    // Check for --help flag to display help
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("Usage: fesh [options]");
        println!("  --version, -v   Show version");
         println!("  --help, -h  Show this help menu");
    }

    loop {
        // Use '>' character as the shell prompt
        // Flush it to ensure it prints before read_line
        print!("> ");
        let _ = stdout().flush();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            // Split the input into the command and the arguments
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command { // Handle shell built-ins
                "cd" => { // cd built-in
                    let new_dir = args.peekable().peek().map_or("/", |x| *x); // Default to '/' if a directory was not provided
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) { // Set current directory  (or / if unspecified)
                        eprintln!("{}", e); // Throw error on failure
                    }

                    previous_command = None;
                },
                "exit" => return, // exit built-in
                command => { // Other commands
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        // There is another command piped behind this one
                        // Prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // There are no more commands piped behind this one
                        // Send output to shell stdout
                        Stdio::inherit()
                    };

                    // Run the command inserted by the user
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // Block until the final command has finished
            let _ = final_command.wait();
        }

    }
}