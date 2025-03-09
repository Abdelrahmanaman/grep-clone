use cli_project::Config;
use std::{env, process::exit};

fn main() {
    //Geting args from the CLI
    let args: Vec<String> = env::args().collect();

    //Build the args eg: path_file and query
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprint!("Error: {}", e);
        exit(1)
    });

    // Run the query and the file_path search
    if let Err(e) = Config::run(config) {
        println!("Application error: {}", e)
    }
}
