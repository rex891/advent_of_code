use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // set initial status
    let mut curr_status = [[false; 100]; 100];
    let mut scratch_status = [[false; 100]; 100];
    for (row_num, line) in input.lines().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c == '#' {
                curr_status[row_num][col_num] = true;
            }
        }
    }
    next_iteration(&curr_status, &scratch_status);
}

fn next_iteration(curr_status: &[[bool; 100]], next_status: &[[bool; 100]]) {
    for row in 0..100 {
        for col in 0..100 {
            let mut count = 0;
            for i in 0..=2 {
                for j in 0..=2 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    if row + i == 0 || col + j == 0 {
                        continue;
                    }
                    if let Some(bool_row) = curr_status.get(row + i - 1) {
                        if let Some(true) = bool_row.get(col + j - 1) {
                            count += 1;
                        }
                    }
                }
            }
            if curr_status[row][col] && !(count == 2 || count == 3) {
                curr_status[row][col] = false
            } else 
        }
    }
}
