use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(config.file_path)?;

        for line in search(&config.query, &content) {
            println!("Query found: {}", line)
        }

        Ok(())
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {

    use std::io;

    use super::*;

    #[test]
    fn find_result() -> Result<(), io::Error> {
        let query = "Hello";

        let content = fs::read_to_string("foo.rs")?;

        assert_eq!(vec!["print!(\"Hello World\")"], search(query, &content));

        Ok(())
    }
}
