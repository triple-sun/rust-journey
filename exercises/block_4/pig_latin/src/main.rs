use std::io;

fn main() {
    println!("Welcome to pig latin converter!");

    'input: loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_string();

        if input.len() == 0 {
            println!("Empty string received!");
            continue 'input;
        }

        let mut words: Vec<&str> = input.split_whitespace().collect();
        let mut latins: Vec<String> = Vec::new();

        'words: for word in words.iter_mut() {
            if word.len() == 0 {
                continue 'words;
            }

            let ch = word.chars().nth(0).unwrap();

            let lat = match ch {
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' => format!("{word}way"),
                _ => {
                    let slice = &word[1..];
                    format!("{slice}{ch}ay")
                }
            };

            latins.push(lat);
        }

        println!("Done: {}", latins.join(" "))
    }
}
