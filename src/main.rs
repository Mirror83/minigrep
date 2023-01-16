use std::env;
use std::fs;

fn main() {
    // The collect function turns an iterator into a collection of the annotated type
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{query}'");
    println!("In '{file_path}'");

    let content = fs::read_to_string(file_path).expect("Could not read the file");

    println!("With content:\n{content}");

}
