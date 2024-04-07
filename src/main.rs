use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = parse_config(args);


	let contents = String::from_utf8(fs::read(&config.file_path).expect("Should have been able to read the file")).expect("Not UTF-8 string");


	println!("Searching for {}", config.query);
	println!("In file {}", config.file_path);
	println!("With text:\n{contents}");
}

struct Config {
	query: String,
	file_path: String,
}

fn parse_config(args: Vec<String>) -> Config {
	let query = args[1].clone();
	let file_path = args[2].clone();
	Config {
		query,
		file_path,
	}
}

