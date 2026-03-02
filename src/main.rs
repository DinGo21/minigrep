use std::{io, env, error::Error, fs, process};

struct Config {
    query: String,
    path: Option<String>,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let path = args.next();
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
    if let Some(path) = config.path {
        exec_grep(&config.query, &fs::read_to_string(path)?, config.ignore_case);
        return Ok(());
    };
    loop {
        let mut content = String::new();
        io::stdin().read_line(&mut content)?;
        if content.is_empty() {
            break;
        }
        exec_grep(&config.query, &content, config.ignore_case);
    }
    Ok(())
}

fn exec_grep(query: &str, content: &str, ignore_case: bool) {
    match ignore_case {
        true => minigrep::grep_case_insensitive(query, content).for_each(|line| println!("{line}")),
        false => minigrep::grep(query, content).for_each(|line| println!("{line}")),
    };
}
