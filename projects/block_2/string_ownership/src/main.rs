fn take_ownership(str: String) -> String {
    String::from(str)
}

fn main() {
    let str = String::from("string");

    let taken = take_ownership(str);

    println!("Took ownership of: {taken}");

    // should not compile
    println!("Now {str} is no longer valid!");
}
