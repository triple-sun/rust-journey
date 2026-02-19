use std::{thread::sleep, time::Duration};

const BASE: &str = "On the first day of Christmas\nMy true love sent to me";

fn main() {
    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!("12 days of christmas");

    for i in 0..12 {
        println!("{BASE}");

        for vi in (0..=i).rev() {
            println!("{}", verses[vi]);
        }

        sleep(Duration::from_secs(1));

        println!("")
    }

    println!("Merry christmas!")
}
