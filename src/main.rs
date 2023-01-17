use std::{env, process};
use minigrep::{Config, run};

fn main() {
    // The collect function turns an iterator into a collection of the annotated type
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // A non-zero exit code signifies erroneous termination
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}
