use crate::read;
use std::{collections::HashMap, panic};

pub fn five() {
    if let Ok(lines) = read::read_lines("./5.txt") {
        let mut comes_after: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut res_count: u32 = 0;

        for line in lines.flatten() {
            if line.contains("|") {
                let mut parts = line.split('|');
                if let (Some(one), Some(two)) = (parts.next(), parts.next()) {
                    if let (Ok(one), Ok(two)) = (one.parse::<u8>(), two.parse::<u8>()) {
                        comes_after.entry(one).or_insert_with(Vec::new).push(two);
                    } else {
                        panic!("u8 arent u8ting");
                    }
                }
            } else if line.contains(',') {
                let mut parts = line.split(',');
                let mut numbers_buffer: Vec<u8> = Vec::new();
                while let Some(curr) = parts.next() {
                    if let Ok(number) = curr.parse::<u8>() {
                        numbers_buffer.push(number);
                    } else {
                        panic!("Failed to parse: {}", curr);
                    }
                }
                res_count += check_order(&comes_after, &numbers_buffer) as u32;
            }
        }
        println!("Result count is: {:?}", res_count);
    }
}

fn check_order(order: &HashMap<u8, Vec<u8>>, line: &Vec<u8>) -> u8 {
    for (index, val) in line.iter().enumerate() {
        if let Some(curr_rules) = order.get(val) {
            for value in curr_rules {
                if let Some(value_index) = line.iter().position(|probe| probe == value) {
                    if value_index < index {
                        return correct_order(&order, &line);
                    }
                }
            }
        }
    }
    return 0;
}

fn correct_order(order: &HashMap<u8, Vec<u8>>, line: &Vec<u8>) -> u8 {
    println!("Trying to correct: {:?}", line);

    let mut corrected_line = line.clone();

    corrected_line.sort_by(|a, b| {
        // If `a` must come after `b`, return `Ordering::Greater`
        if let Some(rules) = order.get(b) {
            if rules.contains(a) {
                return std::cmp::Ordering::Greater;
            }
        }

        // If `b` must come after `a`, return `Ordering::Less`
        if let Some(rules) = order.get(a) {
            if rules.contains(b) {
                return std::cmp::Ordering::Less;
            }
        }

        // Otherwise, leave the relative order unchanged
        std::cmp::Ordering::Equal
    });

    return corrected_line[corrected_line.len() / 2];
}
