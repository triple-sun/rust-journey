use rand::{Rng, RngExt};
use std::collections::HashMap;

fn mean(vec: &Vec<i32>) -> f64 {
    let sum: i32 = vec.iter().sum();
    let len = vec.len();

    sum as f64 / len as f64
}

fn median(vec: &mut Vec<i32>) -> f64 {
    let len: i32 = vec.len() as i32;

    vec.sort();

    if len % 2 == 0 {
        let first_median = vec[vec.len() / 2 - 1];
        let second_median = vec[vec.len() / 2];

        (first_median as f64 + second_median as f64) / 2.0
    } else {
        vec[vec.len() / 2] as f64
    }
}

fn mode(vec: &Vec<i32>) -> Vec<i32> {
    let mut mean: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, u32> = HashMap::new();

    for i in vec.iter() {
        match map.get_mut(i) {
            Some(count) => *count += 1,
            None => {
                map.insert(*i, 1);
            }
        }
    }

    if let Some(max_value) = map.values().max().copied() {
        map.iter()
            .filter(|&(_, &val)| val == max_value)
            .for_each(|(&k, _)| mean.push(k));
    }

    return mean;
}

fn main() {
    let mut test_vec: Vec<i32> = (0..10).map(|_| rand::rng().random_range(-11..11)).collect();

    println!("Test vector is: {:?}", test_vec);

    let test_mean = mean(&test_vec);
    println!("Mean is {test_mean}");

    let test_mode = mode(&test_vec);

    println!("Mode(s) are {:?}", test_mode);
    
    let test_median = median(&mut test_vec);

    println!("Median is {test_median}")
}
