use std::env;
use std::io::{stdout, stdin, Write};
mod input;

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

        // Parse user input
        input::parse_input(&input);
    }
}