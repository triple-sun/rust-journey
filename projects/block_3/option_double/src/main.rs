fn double_option(opt: Option<usize>) -> Option<usize> {
    match opt {
        None => None,
        Some(value) => Some(value * 2),
    }
}

fn main() {
    let mut test_val: Option<usize> = None;

    let double_none = double_option(test_val);

    println!("Double of {:?} is {:?}", test_val, double_none);

    test_val = Some(123);

    let double_some = double_option(test_val);

    println!("Double of {:?} is {:?}", test_val, double_some);
}
