use std::{env, process};
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // As programmers do not expect new to fail, we change the name of the
    // function from new to build
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments were provided")
        } else if args.len() > 3 {
           return Err("Too many arguments were provided")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

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

// This function will either return nothing (the unit type) or
// a dynamic error and let the calling code (in this case, the main function)
// decide on how to handle potential failure
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The ? operator makes the function return early in case of failure
    // and returns the error type that caused the failure to the calling function
    let content = fs::read_to_string(&config.file_path)?;

    println!("Searching for '{}'", config.query);
    println!("In '{}'", config.file_path);

    println!("With content:\n{}", content);

    Ok(())
}
