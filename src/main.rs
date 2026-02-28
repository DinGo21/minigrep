use std::{env, error::Error, fs, process};
use minigrep::{grep, grep_case_insensitive};

struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a path string"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem during configuration: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("Unable to read file: {error}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    match config.ignore_case {
        true => grep_case_insensitive(&config.query, &content).for_each(|line| println!("{line}")),
        false => grep(&config.query, &content).for_each(|line| println!("{line}")),
    };
    Ok(())
}
