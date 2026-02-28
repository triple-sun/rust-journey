use std::fmt::{Display};

use rand::RngExt;

#[derive(Eq, PartialEq, Debug)]
struct ComparableObject {
    value: i8,
}

impl PartialOrd for ComparableObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for ComparableObject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       if self.value > other.value {
        return std::cmp::Ordering::Greater
       } else if self.value < other.value {
        return std::cmp::Ordering::Less
       } else {
        return std::cmp::Ordering::Equal
       }
    }
}

fn largest<T: PartialOrd + std::cmp::Ord>(elements: &Vec<T>) -> Option<&T> {
    return elements.iter().max();
}

fn main() {
    let test_vec: Vec<ComparableObject> = (0..10)
        .map(|_| ComparableObject {
            value: rand::rng().random(),
        })
        .collect();

    println!("Test vector is: {:#?}", test_vec);

    let largest = largest(&test_vec);

    println!("Largest is: {:#?}", largest.unwrap())
}
