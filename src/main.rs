use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	let (query, file_path) = parse_config(&args);


	let contents = String::from_utf8(fs::read(file_path).expect("Should have been able to read the file")).expect("Not UTF-8 string");


	println!("Searching for {}", query);
	println!("In file {}", file_path);
	println!("With text:\n{contents}");
}

struct Config<'a> {
	query: &'a String,
	file_path: &'a String,
}

fn parse_config(args: &Vec<String>) -> Config {
	let query = &args[1];
	let file_path = &args[2];
	Config {
		query,
		file_path,
	}
}

