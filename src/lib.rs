use std::error::Error;
use std::fs;

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str,contents:&'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}


pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Self { query, file_path }
    }


    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })


    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query, contents));

    }

}
