use std::{env, error::Error, fs};
#[cfg(test)]
mod tests;

pub enum Mode {
    IgnoreCase,
    Invalid,
    None,
}

pub struct Config {
    pub mode: Mode,
    pub query: String,
    pub file_path: String,
    pub ignore_case: Option<bool>,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        match args.len() {
            3 => Ok(Config {
                mode: Mode::None,
                query: args.remove(1),
                file_path: args.remove(1),
                ignore_case: match env::var("IGNORE_CASE") {
                    Ok(val) => Some(val.eq("1")),
                    Err(_) => None,
                },
            }),
            4 => Ok(Config {
                mode: match_mode(args.remove(1)),
                query: args.remove(1),
                file_path: args.remove(1),
                ignore_case: match env::var("IGNORE_CASE") {
                    Ok(val) => Some(val.eq("1")),
                    Err(_) => None,
                },
            }),

            0..=2 => {
                return Err("not enough arguments");
            }
            _ => {
                return Err("too many arguments");
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("\n===================");
    println!("With text:\n{contents}");
    println!("===================\n");
    let results = match config.ignore_case {
        Some(true) => search_case_insensitive(&config.query, &contents),
        Some(false) => search(&config.query, &contents),
        None => match config.mode {
            Mode::IgnoreCase => search_case_insensitive(&config.query, &contents),
            _ => search(&config.query, &contents),
        },
    };

    for i in results {
        println!("Matched: {}", i);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim())
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_ascii_lowercase();
    for line in contents.lines() {
        if line.to_ascii_lowercase().contains(&query) {
            results.push(line.trim())
        }
    }
    results
}

fn match_mode(mode: String) -> Mode {
    match mode.as_str() {
        "-i" => Mode::IgnoreCase,
        _ => Mode::Invalid,
    }
}
