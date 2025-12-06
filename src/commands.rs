pub fn commands() {
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
}