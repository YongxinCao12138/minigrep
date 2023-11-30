use std::{env, fs, error::Error};

pub struct Config {
    pub search_string: String,
    pub file_path: String,
    pub is_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        return Ok(Config {
            search_string: args[1].clone(),
            file_path: args[2].clone(),
            is_sensitive: env::var("IGNORE_CASE").is_ok(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let res_lines = if config.is_sensitive {
        search_case_insensitive(&config.search_string, &contents)
    } else {
        search(&config.search_string, &contents)
    };

    for line in res_lines {
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

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let lowercase_query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            result.push(line);
        }
    }


    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
