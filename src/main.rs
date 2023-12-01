use std::{env, process};

use minigrep::Config;

fn main() {
    let config: Result<Config, &str> = Config::build(env::args());

    let config: Config = config.unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
