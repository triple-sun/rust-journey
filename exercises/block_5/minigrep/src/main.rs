use std::{env, error::Error, fs, process};

use minigrep::{Config, search};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error while parsing arguments: {err}");
        process::exit(1);
    });

    println!("Looking for {} in {}", config.query, config.filepath);

    if let Err(e) = run(&config) {
        print!("An error has occured: {e}");
        process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    for (n, line) in search(config, &contents) {
        if config.flags.add_line_numbers {
            println!("{n}. {line}")
        } else {
            println!("{line}")
        }
    }

    Ok(())
}
