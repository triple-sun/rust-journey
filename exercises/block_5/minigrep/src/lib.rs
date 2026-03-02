#[derive(Default)]
pub struct Flags {
    pub reverse: bool,
    pub ignore_case: bool,
    pub add_line_numbers: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags::default()
    }

    pub fn build(flags: &str) -> Flags {
        let mut result = Flags::new();

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

pub struct Config<'a> {
    pub query: &'a str,
    pub filepath: &'a str,
    pub flags: Flags,
}

impl Config<'_> {
    pub fn build<'a>(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() == 0 {
            return Err("Should provide at least 2 args: cargo run minigrep -- [QUERY] [FILEPATH]");
        }

        let has_flags = args[1].starts_with('-');
        let min_arg_count = if has_flags { 4 } else { 3 };

        if args.len() < min_arg_count {
            return Err("Not enough arguments!");
        }

        if has_flags {
            return Ok(Config {
                query: &args[2],
                filepath: &args[3],
                flags: Flags::build(&args[1]),
            });
        }

        return Ok(Config {
            query: &args[1],
            filepath: &args[2],
            flags: Flags::new(),
        });
    }
}

fn does_contain(query: &str, line: &str, ignore_case: bool) -> bool {
    if ignore_case {
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

    for (i, line) in text.lines().enumerate() {
        let contains = does_contain(&config.query, line, config.flags.ignore_case);

        let is_ok = (config.flags.reverse && !contains) || (!config.flags.reverse && contains);

        if is_ok {
            result.push((i + 1, &line));
        }
    }

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
    fn searches() {
        let config = Config {
            query: "duct",
            filepath: "",
            flags: Flags::new(),
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
        let mut flags = Flags::new();

        flags.ignore_case = true;

        let config = Config {
            query: "Duct",
            filepath: "",
            flags,
        };

        let contents = "\
Rust:
safe, fast, produCtive.
Pick three.";

        assert_eq!(vec![(2, "safe, fast, produCtive.")], search(&config, contents));
    }

    #[test]
    fn searches_reverse() {
        let mut flags = Flags::new();

        flags.reverse = true;

        let config = Config {
            query: "duct",
            filepath: "",
            flags,
        };

        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(1, "Rust:"), (3, "Pick three.")], search(&config, contents));
    }
}
