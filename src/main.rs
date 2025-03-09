use std::{env, fs};
fn main() {
    //Geting args from the CLI
    let args: Vec<String> = env::args().collect();

    let Config { query, file_path } = parse_config(&args);
    print!("{} {}", query, file_path);

    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("\n{}", content);
}

struct Config {
    query: String,
    file_path: String,
}
fn parse_config(args: &Vec<String>) -> Config {
    let query: String = args[1].clone();
    let file_path: String = args[2].clone();

    Config { query, file_path }
}
