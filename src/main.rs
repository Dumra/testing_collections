use std::collections::HashMap;
use std::f32::INFINITY;

fn main() {
    let mut vector = vec![1, 5, -8, -25, 64];

    println!("Input Vector: {:?}", vector);
    println!("Mean is: {}", calculate_mean(&vector));
    println!("Median is: {}", calculate_median(&mut vector));

    let mut vector = vec![1, 5, -8, 35, -25, 64];
    println!("Median is: {}", calculate_median(&mut vector));

    let vector = vec![1, 5, 35, 32, 1, 3, 8, 4, 1, 32, -8, 35, -25, 64];

    println!("Vector: {:?}", vector);
    let mode = calculate_mode(&vector);
    println!("The mode is {}, with counts: {} ", mode.0, mode.1);
}

fn calculate_mean(vector: &Vec<i32>) -> i32 {
    let mut sum = 0i32;
    let count = vector.len() as i32;
    for value in vector {
        sum += *value;
    }

    sum / count
}

fn calculate_median(vector: &mut Vec<i32>) -> i32 {
    vector.sort();
    println!("Sorted vector: {:?}", vector);
    let count = vector.len() as u32;
    let median: i32;
    if count % 2 != 0 {
        let index = (count as f32 / 2.0).round() - 1.0;
        median = get_value_from_vector(index as usize, &vector);
    } else {
        let index = (count / 2) - 1;
        let first_middle_value = get_value_from_vector(index as usize, &vector);
        let second_middle_value = get_value_from_vector((index + 1) as usize, &vector);
        median = (first_middle_value + second_middle_value) / 2;
    }

    median
}

fn calculate_mode(vector: &Vec<i32>) -> (i32, u32) {
    let mut map: HashMap<i32, u32> = HashMap::new();
    for value in vector {
        let count = map.entry(*value).or_insert(0);
        *count += 1;
    }
    let mut mode = (0, 0);
    for (value, counts) in &map {
        if *counts > mode.1 {
            mode = (*value, *counts);
        }
    }

    mode
}

fn get_value_from_vector(index: usize, vector: &Vec<i32>) -> i32 {
    match vector.get(index) {
        Some(value) => *value,
        None => {
            println!("Can't retrieve value from the vector");
            INFINITY as i32
        }
    }
}
