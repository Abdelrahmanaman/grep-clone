use ansi_term::Color;
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

        for (line_num, line) in search(&config.query, &content) {
            let highlighted = line.replace(
                &config.query,
                &Color::Yellow.bold().paint(&config.query).to_string(),
            );
            println!("Query found in line {}: {}", line_num, highlighted);
        }
        Ok(())
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<(i32, &'a str)> {
    let mut results: Vec<(i32, &'a str)> = Vec::new();
    let mut line_num = 1;

    for line in content.lines() {
        if line.contains(query) {
            results.push((line_num, line.trim()));
        }
        line_num += 1;
    }
    print!("{:?}", results);
    results
}

#[cfg(test)]
mod tests {

    use std::io;

    use super::*;

    #[test]
    fn find_result() -> Result<(), io::Error> {
        let query = "fo";

        let content = fs::read_to_string("foo.test.rs")?;

        assert_eq!(vec![(2, "print!(\"Hello foo\")")], search(query, &content));

        Ok(())
    }
}
