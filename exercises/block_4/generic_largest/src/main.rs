use rand::RngExt;

#[derive(Eq, PartialEq, Debug)]
struct ComparableObject {
    value: i8,
}

impl PartialOrd for ComparableObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableObject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value > other.value {
            std::cmp::Ordering::Greater
        } else if self.value < other.value {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn largest<T: PartialOrd + std::cmp::Ord>(elements: &[T]) -> Option<&T> {
    elements.iter().max()
}

fn main() {
    let test_vec: Vec<ComparableObject> = (0..10)
        .map(|_| ComparableObject {
            value: rand::rng().random(),
        })
        .collect();

    println!("Test obj vector is: {:#?}", test_vec);

    let largest_obj = largest(&test_vec);

    println!("Largest obj is: {:#?}", largest_obj.unwrap());
}
