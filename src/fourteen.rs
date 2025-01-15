use crate::read;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Vec2,
}

pub fn fourteen() {
    let mut robot_vec: Vec<Robot> = Vec::new();
    let width = 101 as u8;
    let height = 103 as u8;
    let cycles = 10000 as u32;
    let mut quadrant_map: HashMap<u8, u32> = HashMap::new();
    let mut final_pos_robot: Vec<Robot> = Vec::new();

    if let Ok(lines) = read::read_lines("./14.txt") {
        for line in lines.flatten() {
            let mut parts = line.split(' ');
            let mut pos = parts.next().map(|x| x[2..].split(",")).unwrap();
            let mut velocity = parts.next().map(|x| x[2..].split(",")).unwrap();

            if let (Some(x), Some(y), Some(vel_x), Some(vel_y)) =
                (pos.next(), pos.next(), velocity.next(), velocity.next())
            {
                let position = Position {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                };
                let velocity = Vec2 {
                    x: vel_x.parse().unwrap(),
                    y: vel_y.parse().unwrap(),
                };

                robot_vec.push(Robot {
                    position: position.clone(),
                    velocity: velocity.clone(),
                });

                let updated_position =
                    calculate_position(&position, &velocity, cycles, width, height);

                let robot = Robot {
                    position: updated_position,
                    velocity,
                };

                if robot.position.x == width / 2 || robot.position.y == height / 2 {
                    println!("Middle lines, skipping: {:?}", robot);
                    continue;
                } else {
                    let quadrant_number = assign_quadrant(&robot.position, width, height);
                    quadrant_map
                        .entry(quadrant_number)
                        .and_modify(|value| *value += 1)
                        .or_insert(1);
                }

                final_pos_robot.push(robot);
            }
        }

        println!("Map: {:?}", quadrant_map);

        let mut res_count = 0;
        for (_, value) in quadrant_map.iter() {
            if res_count == 0 {
                res_count = *value;
                continue;
            }
            res_count *= value;
        }
        println!("Result: {:?}", res_count);

        let best_cycle = find_best_cycle_by_variance(&mut robot_vec, width, height, cycles);

        println!("Best cycle with the least entropy: {}", best_cycle);
    }
}

fn calculate_position(pos: &Position, vel: &Vec2, cycles: u32, width: u8, height: u8) -> Position {
    let (dif_x, dif_y) = (
        (vel.x * cycles as i32) as i16,
        (vel.y * cycles as i32) as i16,
    );

    let new_x = modulo_u8(pos.x as i16 + dif_x, width as i16);
    let new_y = modulo_u8(pos.y as i16 + dif_y, height as i16);

    Position {
        x: new_x as u8,
        y: new_y as u8,
    }
}

fn assign_quadrant(pos: &Position, width: u8, height: u8) -> u8 {
    let x_middle = width / 2;
    let y_middle = height / 2;

    if pos.x < x_middle && pos.y < y_middle {
        return 1;
    } else if pos.x > x_middle && pos.y < y_middle {
        return 2;
    } else if pos.x < x_middle && pos.y > y_middle {
        return 3;
    } else {
        return 4;
    }
}

fn calculate_variance(positions: &Vec<i32>) -> f64 {
    let mean = positions.iter().sum::<i32>() as f64 / positions.len() as f64;
    let sum_squared_diff: f64 = positions
        .iter()
        .map(|&pos| (pos as f64 - mean).powi(2))
        .sum();
    sum_squared_diff / positions.len() as f64
}

fn find_best_cycle_by_variance(
    robot_vec: &mut Vec<Robot>,
    width: u8,
    height: u8,
    max_cycles: u32,
) -> u32 {
    let mut min_variance = f64::MAX;
    let mut best_cycle = 1;

    for cycle in 1..=max_cycles {
        // Collect the positions along the X and Y axes
        let mut x_positions = Vec::new();
        let mut y_positions = Vec::new();

        for robot in robot_vec.iter_mut() {
            robot.position =
                calculate_position_part2(&robot.position, &robot.velocity, width, height);
            x_positions.push(robot.position.x as i32);
            y_positions.push(robot.position.y as i32);
        }

        // Calculate variance along X and Y axes
        let x_variance = calculate_variance(&x_positions);
        let y_variance = calculate_variance(&y_positions);

        // Combine the variances (can be a simple sum or weighted sum)
        let total_variance = x_variance + y_variance;

        // Track the cycle with the minimum variance
        if total_variance < min_variance {
            min_variance = total_variance;
            best_cycle = cycle;
        }
    }

    best_cycle
}

fn calculate_position_part2(pos: &Position, vel: &Vec2, width: u8, height: u8) -> Position {
    let new_x = modulo_u8(pos.x as i16 + vel.x as i16, width as i16);
    let new_y = modulo_u8(pos.y as i16 + vel.y as i16, height as i16);

    Position {
        x: new_x as u8,
        y: new_y as u8,
    }
}

fn modulo_u8(value: i16, max: i16) -> i16 {
    let result = ((value % max) + max) % max;
    if result == 0 && value != 0 {
        max
    } else {
        result
    }
}
