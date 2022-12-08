use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let x: u32 = input
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("A", "X") => 3 + 1,
            ("A", "Y") => 6 + 2,
            ("A", "Z") => 0 + 3,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 6 + 1,
            ("C", "Y") => 0 + 2,
            ("C", "Z") => 3 + 3,
            _ => 0,
        })
        .sum();
    dbg!(x);
    let x: u32 = input
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            _ => 0,
        })
        .sum();
    dbg!(x);
}
