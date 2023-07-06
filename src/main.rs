use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguements, {}", err);
        process::exit(0);
    });
    println!("Searching for {} in {}", config.query, config.file);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error, {}", e);
        process::exit(1);
    }
}
