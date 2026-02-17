use std::{io, process::exit};

fn main() {
    println!("Please input a number");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            if n == "quit" {
                exit(0);
            }

            return;
        }
    };

    let mut sequence: Vec<i64> = [0, 1].to_vec();

    println!("Calculating fibonacci number #{n}...");

    if n <= 1 {
        println!("Fibonacci number #{n} is {:?}", sequence.get(n).unwrap());
        return;
    }

    for i in 2..=n {
        println!("{}", sequence[sequence.len() - 1]);
        sequence.push(sequence[i - 1] + sequence[i - 2]);
    }

    println!("Fibonacci number #{n} is {:?}", sequence.get(n).unwrap())
}
