use std::io;

enum Unit {
    Celsius,
    Fahrenheit,
}

fn ftoc(f: f64) -> f64 {
    (f - 32.0) * 0.5556
}

fn ctof(c: f64) -> f64 {
    c / 0.5556 + 32.0
}

fn main() {
    println!("Type `c` for celsius to fahrenheit or `f` for fahrenheit to celsius conversion");
    println!("Type `q` to quit");

    let mut unit_input: String;
    let mut unit: Option<Unit> = None;

    loop {
        if unit.is_none() {
            unit_input = String::new();

            io::stdin()
                .read_line(&mut unit_input)
                .expect("Failed to read line!");

            unit = match unit_input.trim() {
                "f" => Some(Unit::Fahrenheit),
                "c" => Some(Unit::Celsius),
                "q" => {
                    break;
                }
                _ => {
                    println!("Please type `c`, `f` or `q`.");
                    continue;
                }
            };
        } else {
            let mut deg = String::new();

            println!("Type in degrees to get converted value");

            io::stdin()
                .read_line(&mut deg)
                .expect("Failed to read line!");

            let deg: f64 = match deg.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number");
                    continue;
                }
            };

            match unit.unwrap() {
                Unit::Celsius => {
                    let result = ctof(deg);
                    println!("{deg}C is {result:.2}F");
                }
                Unit::Fahrenheit => {
                    let result = ftoc(deg);
                    println!("{deg}F is {result:.2}C")
                }
            }

            break;
        }
    }
}
