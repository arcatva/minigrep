use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    for i in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
	use super::*;

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

