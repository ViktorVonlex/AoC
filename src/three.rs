use crate::read;

pub fn three() {
    if let Ok(lines) = read::read_lines("./3.txt") {
        let mut calls: Vec<(String, Vec<i32>)> = Vec::new();
        let mut mul_enabled = true;

        for input in lines.flatten() {
            println!("{:?}", input);
            let mut pos = 0;

            while pos < input.len() {
                if input[pos..].starts_with("mul(") && mul_enabled {
                    pos += 4; // Skip past "mul("

                    let mut args = Vec::new();
                    let mut current_arg = String::new();
                    let mut valid = true;
                    let mut paren_count = 1; // Track parentheses

                    while pos < input.len() && paren_count > 0 {
                        match input.chars().nth(pos) {
                            Some('0'..='9') => {
                                current_arg.push(input[pos..].chars().next().unwrap());
                            }
                            Some(',') => {
                                if let Ok(num) = current_arg.parse::<i32>() {
                                    args.push(num);
                                    current_arg.clear();
                                } else {
                                    valid = false;
                                    break;
                                }
                            }
                            Some('(') => {
                                // Invalid nested parentheses in arguments
                                valid = false;
                                break;
                            }
                            Some(')') => {
                                if let Ok(num) = current_arg.parse::<i32>() {
                                    args.push(num);
                                    current_arg.clear();
                                }
                                paren_count -= 1;
                                pos += 1; // Move past ')'
                                break;
                            }
                            Some(_) => {
                                valid = false;
                                break;
                            }
                            None => break,
                        }
                        pos += 1;
                    }

                    // Add the call if valid
                    if valid && paren_count == 0 && args.len() == 2 {
                        calls.push(("mul".to_string(), args));
                    }
                } else if input[pos..].starts_with("do()") {
                    mul_enabled = true;
                    pos += 4;
                } else if input[pos..].starts_with("don't()") {
                    mul_enabled = false;
                    pos += 7;
                } else {
                    pos += 1;
                }
            }
        }

        println!("{:?}", calls);

        let mut result = 0;
        for call in calls.iter() {
            let mut multiplication_res = 1;
            call.1
                .iter()
                .for_each(|val| multiplication_res = multiplication_res * val);
            result = result + multiplication_res;
        }
        println!("Multiplication result: {:?}", result);
    }
}
