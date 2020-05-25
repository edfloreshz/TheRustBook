//  Created by Eduardo Flores on 5/18/20.
//  edfloreshz@gmail.com

// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn test_grades() {
    let grades = vec![
        10, 10, 10, 9, 9, 8, 8, 8, 8, 8, 7, 7, 10, 9, 6, 7, 8, 8, 9, 10,
    ];
    let mean = mean(&grades);
    let median = median(&grades);
    let mode = mode(&grades);
    println!("Mean: {} \nMedian: {} \nMode: {}", mean, median, mode);
}

pub fn mean(grades: &Vec<i32>) -> i32 {
    grades.iter().sum::<i32>() / grades.len() as i32
}

pub fn median(grades: &Vec<i32>) -> i32 {
    let mut sorted_grades = grades.clone();
    sorted_grades.sort();
    match sorted_grades.len() % 2 {
        0 => {
            let middle = sorted_grades.len() / 2;
            (sorted_grades[middle] + sorted_grades[middle + 1]) / 2
        }
        _ => sorted_grades[sorted_grades.len() / 2],
    }
}

pub fn mode(grades: &Vec<i32>) -> i32 {
    let mut hashed_grades = HashMap::new();
    let mut mode = (0, 0);
    let mut max = 0;
    for grade in grades {
        *hashed_grades.entry(*grade).or_insert(0) += 1;
    }
    for (key, value) in hashed_grades {
        if value > max {
            max = value;
            mode = (key, value);
        }
    }
    mode.0
}
