use crate::read;

pub fn two() {
    let mut safe_count = 0;

    if let Ok(lines) = read::read_lines("./2.txt") {
        for line in lines.flatten() {
            let parts = line.split(" ");
            let nums: Vec<i32> = parts.filter_map(|s| s.parse().ok()).collect();

            if is_safe(&nums) {
                safe_count += 1;
                println!("Safe (no removal): {:?}", nums);
                continue;
            }

            let mut can_be_safe = false;
            for i in 0..nums.len() {
                let mut modified_nums = nums.clone();
                modified_nums.remove(i);

                if is_safe(&modified_nums) {
                    can_be_safe = true;
                    break;
                }
            }

            if can_be_safe {
                safe_count += 1;
                println!("Safe (with removal): {:?}", nums);
            } else {
                println!("Unsafe: {:?}", nums);
            }
        }
    }

    println!("Safe count: {:?}", safe_count);
}

fn is_safe(nums: &Vec<i32>) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let mut climbing = false;
    let mut set_direction = false;

    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        let diff_abs = diff.abs();

        if diff_abs == 0 || diff_abs > 3 {
            return false;
        }

        if !set_direction {
            if diff > 0 {
                climbing = true;
            }
            set_direction = true;
        }

        if climbing && (diff < 0) {
            return false;
        } else if !climbing && (diff > 0) {
            return false;
        }
    }

    return true;
}
