use std::{env, process};
use website_blocker as wb;

fn main() {
    let arguments = env::args();

    let config = wb::config::Config::build(arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = wb::run(&config) {
        eprintln!("Application Error: {}", err);
        process::exit(2);
    }
}
