use std::{fs, error::Error};

pub struct Config {
    pub search_string: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        return Ok(Config {
            search_string: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.search_string, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for content in contents.lines() {
        if content.contains(query) {
            result.push(content);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
