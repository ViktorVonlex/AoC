use std::collections::HashSet;

use crate::read;

pub fn four() {
    if let Ok(lines) = read::read_lines("./4.txt") {
        // Part one
        let mut xmas_count = 0;
        let mut horizontal_count = 0;
        let mut vertical_count = 0;
        let mut diagonal_right_count = 0;
        let mut diagonal_left_count = 0;
        let mut rows: Vec<Vec<char>> = Vec::new();
        let mut possible_sequences: HashSet<String> = HashSet::new();
        possible_sequences.insert("XMAS".to_string());
        possible_sequences.insert("SAMX".to_string());

        for line in lines.flatten() {
            let mut row: Vec<char> = Vec::new();
            line.chars().for_each(|ch| row.push(ch));
            rows.push(row);
        }

        for i in 0..rows.len() {
            for j in 0..rows[i].len() {
                let current_char = rows[i][j];

                if current_char == 'X' || current_char == 'S' {
                    horizontal_count += horizontal_lookahead(&rows, i, j);
                    vertical_count += vertical_lookahead(&rows, i, j);
                    diagonal_right_count += diagonal_rigth_lookahead(&rows, i, j);
                    diagonal_left_count += diagonal_left_lookahead(&rows, i, j);
                }
            }
        }

        println!("Count in horizontal lookahead: {:?}", horizontal_count);
        println!("Count in vertical lookahead: {:?}", vertical_count);
        println!(
            "Count in diagonal_right lookahead: {:?}",
            diagonal_right_count
        );
        println!(
            "Count in diagonal_left lookahead: {:?}",
            diagonal_left_count
        );

        xmas_count = horizontal_count + vertical_count + diagonal_right_count + diagonal_left_count;
        println!("Complete count: {:?}", xmas_count);

        // Part two :(((
        let mut x_count = 0;

        for i in 0..rows.len() {
            for j in 0..rows[i].len() {
                let current_char = rows[i][j];

                if current_char == 'A' {
                    if x_lookahead(&rows, i, j) {
                        x_count += 1;
                    }
                }
            }
        }

        println!("X count: {:?}", x_count);
    }
}

fn horizontal_lookahead(rows: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut xmas_count = 0;

    // Ensure there's enough space to look ahead (4 characters forward)
    if (x + 4) <= rows[y].len() {
        let mut char_buffer = String::from("");

        // Look along x axis
        for i in x..(x + 4) {
            let current_char_inner = rows[y][i];
            char_buffer.push(current_char_inner);
        }

        // Check if the collected string matches any of the patterns
        if char_buffer == "XMAS" || char_buffer == "SAMX" {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn vertical_lookahead(rows: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut xmas_count = 0;

    // Ensure there's enough space to look ahead (4 characters down)
    if (y + 4) <= rows.len() {
        let mut char_buffer = String::from("");

        // Look along y axis
        for i in y..(y + 4) {
            let current_char_inner = rows[i][x];
            char_buffer.push(current_char_inner);
        }

        // Check if the collected string matches any of the patterns
        if char_buffer == "XMAS" || char_buffer == "SAMX" {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn diagonal_rigth_lookahead(rows: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut xmas_count = 0;

    // Ensure there's enough space to look ahead (4 characters down and right)
    if (y + 4) <= rows.len() && (x + 4) <= rows[0].len() {
        let mut char_buffer = String::from("");

        // Look along diagonal in the right direction, x and y increasing
        for i in 0..4 {
            let diagonal_x = x + i;
            let diagonal_y = y + i;
            let current_char_inner = rows[diagonal_y][diagonal_x];
            char_buffer.push(current_char_inner);
        }

        // Check if the collected string matches any of the patterns
        if char_buffer == "XMAS" || char_buffer == "SAMX" {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn diagonal_left_lookahead(rows: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut xmas_count = 0;

    // Ensure there's enough space to look ahead (4 characters down and 4 characters left)
    if (y + 4) <= rows.len() && x >= 3 {
        let mut char_buffer = String::from("");

        // Look along diagonal in the left direction, x decreasing while y increasing
        for i in 0..4 {
            let diagonal_x = x - i;
            let diagonal_y = y + i;
            let current_char_inner = rows[diagonal_y][diagonal_x];
            char_buffer.push(current_char_inner);
        }

        // Check if the collected string matches any of the patterns
        if char_buffer == "XMAS" || char_buffer == "SAMX" {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn x_lookahead(rows: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    // Ensure there's enough space to look ahead (1 character all around)
    if y > 0 && y + 1 < rows.len() && x > 0 && x + 1 < rows[y].len() {
        let mut char_buffer_left = String::new();
        let mut char_buffer_right = String::new();

        // Look around "a"
        char_buffer_right.push(rows[y - 1][x - 1]);
        char_buffer_right.push('A');
        char_buffer_right.push(rows[y + 1][x + 1]);

        char_buffer_left.push(rows[y - 1][x + 1]);
        char_buffer_left.push('A');
        char_buffer_left.push(rows[y + 1][x - 1]);

        // Check if the collected string matches any of the patterns
        if (char_buffer_left == "MAS" || char_buffer_left == "SAM")
            && (char_buffer_right == "MAS" || char_buffer_right == "SAM")
        {
            return true;
        }
    }

    return false;
}
