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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	for line in contents.lines() {
		if line.contains(query) {}
	}
	return;
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

