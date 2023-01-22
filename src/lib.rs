use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // As programmers do not expect new to fail, we change the name of the
    // function from new to build
    pub fn build(args: &[String]) -> Result<Config, &str> {
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

// This function will either return nothing (the unit type) or
// a dynamic error and let the calling code (in this case, the main function)
// decide on how to handle potential failure
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The ? operator makes the function return early in case of failure
    // and returns the error type that caused the failure to the calling function
    let content = fs::read_to_string(&config.file_path)?;

    println!("Searching for '{}'", config.query);
    println!("In '{}'\n", config.file_path);

    let matching_strings = search(&content, &config.query);

    if matching_strings.len() == 0 {
        println!("No matching strings found.")
    } else {
        for line in matching_strings {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str>{
    let mut matching_lines:Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.contains(query) {
            matching_lines.push(line);
        }
    }

    return matching_lines
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search() {
        let contents = "\
Rust:
safe, productive, efficient.
Pick three";
        assert_eq!(vec!["safe, productive, efficient."], search(&contents, "prod"));
    }

}