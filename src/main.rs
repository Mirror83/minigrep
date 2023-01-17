use std::env;
use std::fs;

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query, file_path
        }
    }
}

fn main() {
    // The collect function turns an iterator into a collection of the annotated type
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("{:?}", config);

    println!("Searching for '{}'", config.query);
    println!("In '{}'", config.file_path);

    let content = fs::read_to_string(config.file_path)
        .expect("Could not read the file");

    println!("With content:\n{}", content);

}
