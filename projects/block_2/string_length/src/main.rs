fn get_string_length(string: &String) -> usize {
    string.len()
}

fn main() {
    let string = String::from("Get length from this string");

    let length = get_string_length(&string);

    println!("The length of `{string}` is {length}!");
}
