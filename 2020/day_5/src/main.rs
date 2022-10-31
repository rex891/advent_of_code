use std::fs;

fn main() {
    let mut max = 0;
    let mut seats = [false; 128 * 8];

    for seat in fs::read_to_string("input.txt").unwrap().lines() {
        let (row, col) = seat.split_at(7);
        let (r, c) = (get_row(row), get_col(col));

        let seat_no = r * 8 + c;
        seats[seat_no] = true;
        max = max.max(seat_no);
    }

    println!("part1: {}", max);

    println!(
        "part2: {}",
        (seats
            .iter()
            .enumerate()
            .skip_while(|(_, &v)| !v)
            .skip_while(|(_, &v)| v))
        .next()
        .unwrap()
        .0
    );
}

fn get_row(row: &str) -> usize {
    row.chars().enumerate().fold(0, |acc, (i, c)| {
        acc + (if c == 'B' {
            usize::pow(2, 6 - i as u32)
        } else {
            0
        })
    })
}

fn get_col(col: &str) -> usize {
    col.chars().enumerate().fold(0, |acc, (i, c)| {
        acc + (if c == 'R' {
            usize::pow(2, 2 - i as u32)
        } else {
            0
        })
    })
}
