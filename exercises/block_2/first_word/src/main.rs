fn get_first_word(str: &str) -> &str {
    for (i, &item) in String::from(str).as_bytes().iter().enumerate() {
        if item == b' ' {
            return &str[0..i]
        }
    }

    &str[..]
}

fn main() {
    const TEST_STRING: &str = "this is a test string";

    let test_first_word = get_first_word(TEST_STRING);

    println!("The first word of `{TEST_STRING}` is {test_first_word}!")
}
