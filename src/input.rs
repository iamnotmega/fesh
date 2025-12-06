pub fn parse_input(input: &str) { 
    // Must be peekable so we know when we are on the last command
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
                let root = std::path::Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(&root) { // Set current directory  (or / if unspecified)
                    eprintln!("{}", e); // Throw error on failure
                }

                previous_command = None;
            },
            "exit" => std::process::exit(0), // exit built-in
            command => { // Other commands
                let stdin = previous_command
                    .map_or(
                        std::process::Stdio::inherit(),
                        |output: std::process::Child| std::process::Stdio::from(output.stdout.unwrap())
                    );

                let stdout = if commands.peek().is_some() {
                    // There is another command piped behind this one
                    // Prepare to send output to the next command
                    std::process::Stdio::piped()
                } else {
                    // There are no more commands piped behind this one
                    // Send output to shell stdout
                    std::process::Stdio::inherit()
                };

                // Run the command inserted by the user
                let output = std::process::Command::new(command)
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