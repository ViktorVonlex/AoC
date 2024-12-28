use std::collections::HashMap;
use std::time::Instant;

use crate::read;

pub fn one() {
    let start_time = Instant::now(); // Start timer
                                     // Part one
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    // for part two
    let mut val_count: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read::read_lines("./1.txt") {
        for line in lines.flatten() {
            let mut parts = line.split("   ");
            let first_part = parts.next().unwrap();
            let second_part = parts.next().unwrap();
            let first_num: i32 = first_part.parse().unwrap();
            let second_num: i32 = second_part.parse().unwrap();
            first_column.push(first_num);
            second_column.push(second_num);

            *val_count.entry(second_num).or_insert(0) += 1;
        }
        first_column.sort();
        second_column.sort();

        let mut total_diff = 0;

        for i in 0..first_column.len() {
            total_diff = total_diff + (second_column[i] - first_column[i]).abs();
        }
        println!("Total difference: {:?}", total_diff);

        // Part two

        let mut similarity_score = 0;

        for value in first_column.iter() {
            similarity_score = similarity_score + (value * val_count.get(value).unwrap_or(&0))
        }
        println!("Similarity score: {:?}", similarity_score);
        let elapsed_time = start_time.elapsed(); // Measure elapsed time
        println!("Function executed in: {:.2?}", elapsed_time);
    }
}
