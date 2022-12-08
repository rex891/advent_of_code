use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut calories = input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .collect::<Vec<_>>();

    calories.sort();
    calories.reverse();
    println!("Part 1: {}", calories[0]);

    let top3: i32 = calories.iter().take(3).sum();
    println!("Part 2: {top3}");
}
