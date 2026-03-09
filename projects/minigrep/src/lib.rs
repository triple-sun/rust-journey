use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    NoQuery,
    NoFilePath,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NoQuery => write!(f, "Didn't get a query string!"),
            ConfigError::NoFilePath => write!(f, "Didn't get a file path"),
        }
    }
}

#[derive(Default)]
pub struct Flags {
    pub reverse: bool,
    pub ignore_case: bool,
    pub add_line_numbers: bool,
}

impl Flags {
    pub fn build(flags: &str) -> Flags {
        let mut result = Flags::default();

        if flags.len() < 2 {
            return result;
        }

        for flag in flags.chars() {
            match flag {
                'i' => result.ignore_case = true,
                'r' => result.reverse = true,
                'n' => result.add_line_numbers = true,
                _ => (),
            };
        }

        result
    }
}

#[derive(Default)]
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub flags: Flags,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        args.next();

        let mut config = Config::default();

        let mut has_flags = false;

        match args.next() {
            Some(arg) => {
                if !arg.starts_with("-") {
                    config.query = arg;
                } else {
                    config.flags = Flags::build(&arg);
                    has_flags = true;
                }
            }
            None => return Err(ConfigError::NoFilePath),
        };

        match args.next() {
            Some(arg) => {
                if !has_flags {
                    config.filepath = arg;
                } else {
                    config.query = arg;
                }
            }
            None => {
                if has_flags {
                    return Err(ConfigError::NoQuery);
                }

                return Err(ConfigError::NoFilePath);
            }
        };

        if !has_flags {
            return Ok(config);
        }

        match args.next() {
            Some(arg) => config.filepath = arg,
            None => return Err(ConfigError::NoFilePath),
        };

        Ok(config)
    }
}

fn does_contain(query: &str, line: &str, ignore_case: &bool) -> bool {
    if *ignore_case {
        return line.to_lowercase().contains(&query.to_lowercase());
    }

    line.contains(query)
}

pub fn format_line(i: usize, line: &str, add_line_numbers: bool) -> String {
    if add_line_numbers {
        return format!("{i}. {line}");
    }

    line.to_string()
}

pub fn search<'a>(config: &Config, text: &'a str) -> Vec<(usize, &'a str)> {
    let mut result: Vec<(usize, &'a str)> = Vec::new();

    let ignore_case = config.flags.ignore_case;
    let reverse = config.flags.reverse;

    text.lines().enumerate().for_each(|(i, line)| {
        let contains = does_contain(&config.query, line, &ignore_case);

        let is_ok = (reverse && !contains) || (!reverse && contains);

        if is_ok {
            result.push((i + 1, line))
        };
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_with_no_flags() {
        let args_flags = "--";

        let flags = Flags::build(args_flags);

        assert_eq!(flags.add_line_numbers, false);
        assert_eq!(flags.ignore_case, false);
        assert_eq!(flags.reverse, false);
    }

    #[test]
    fn builds_correct_flags() {
        let args_flags = "-irn";

        let flags = Flags::build(args_flags);

        assert_eq!(flags.add_line_numbers, true);
        assert_eq!(flags.ignore_case, true);
        assert_eq!(flags.reverse, true);
    }

    #[test]
    fn builds_config() {
        let args = vec![
            "--".to_string(),
            "test_query".to_string(),
            "./folder/file.txt".to_string(),
        ];

        let config = Config::build(args.into_iter()).unwrap();

        assert_eq!(config.query, "test_query");
        assert_eq!(config.filepath, "./folder/file.txt")
    }

    #[test]
    fn searches() {
        let config = Config {
            query: "duct".to_string(),
            filepath: "".to_string(),
            flags: Flags::default(),
        };

        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(&config, contents)
        );
    }

    #[test]
    fn searches_insensitive() {
        let mut flags = Flags::default();

        flags.ignore_case = true;

        let config = Config {
            query: "Duct".to_string(),
            filepath: "".to_string(),
            flags,
        };

        let contents = "\
Rust:
safe, fast, produCtive.
Pick three.";

        assert_eq!(
            vec![(2, "safe, fast, produCtive.")],
            search(&config, contents)
        );
    }

    #[test]
    fn searches_reverse() {
        let mut flags = Flags::default();

        flags.reverse = true;

        let config = Config {
            query: "duct".to_string(),
            filepath: "".to_string(),
            flags,
        };

        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(1, "Rust:"), (3, "Pick three.")],
            search(&config, contents)
        );
    }
}
