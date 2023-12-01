use std::{env, error::Error, fs};

pub struct Config {
    pub search_string: String,
    pub file_path: String,
    pub is_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        // get search string
        let search_string = args.next().ok_or("Didn't get a query string arg")?;

        // get search string
        let file_path = args.next().ok_or("Didn't get a file_path arg")?;

        // get is sensitive arg
        let is_sensitive = env::var("IGNORE_CASE").map_or_else(
            |_| {
                // none
                args.any(|arg| arg.eq("-i") || arg.eq("--ignore-case"))
            },
            // Ok
            |env_var| env_var.eq("true") || env_var.eq("1"),
        );

        return Ok(Config {
            search_string,
            file_path,
            is_sensitive,
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
