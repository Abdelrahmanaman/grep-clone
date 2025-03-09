use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
    pub fn run(config: Config) -> Result<(), &'static str> {
        let content = fs::read_to_string(config.file_path).expect("So");
        let query = config.query;
        println!("The query {} in the file {}", query, content);

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
    use super::*;

    #[test]
    fn find_result() {
        let query = "Hello";
        let content = fs::read_to_string("foo.rs").expect("Failed to read file");
        assert_eq!(vec!["print!(\"Hello World\")"], search(query, &content))
    }
}
