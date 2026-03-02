pub struct Config<'a> {
    pub query: &'a str,
    pub filepath: &'a str,
}

impl Config<'_> {
    pub fn build<'a>(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = &args.get(1).expect("Should have query arg!");
        let filepath = &args.get(1).expect("Should have file path arg!");

        Ok(Config { query, filepath })
    }
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_config() {
        let args = Vec::from([
            "--".to_string(),
            "test_query".to_string(),
            "./folder/file.txt".to_string(),
        ]);

        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "test_query");
        assert_eq!(config.filepath, "./folder/file.txt")
    }

    #[test]
    fn finds_query() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
