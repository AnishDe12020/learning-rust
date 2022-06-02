use std::env;
use std::process;

use rustygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = rustygrep::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}