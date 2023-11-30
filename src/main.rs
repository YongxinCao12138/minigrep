use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Result<Config, &str> = Config::build(&args);

    let config: Config = config.unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    
    println!("search string:{}", config.search_string);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("application error: {e}");
        process::exit(1);
    }
}
