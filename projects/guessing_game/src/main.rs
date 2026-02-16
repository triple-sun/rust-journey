use std::{cmp::Ordering, io};

use rand::RngExt;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    let mut attempt = 0;

    loop {
        println!("Please input your guess.");

        if attempt % 5 == 0 {
            println!("Reminder: Type `quit` to quit");
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        attempt += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess == "quit" {
                    break;
                }
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
