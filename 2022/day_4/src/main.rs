use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // part 1
    let num_contains = input
        .lines()
        .filter(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();
            let a1: u32 = a1.parse().unwrap();
            let a2: u32 = a2.parse().unwrap();
            let b1: u32 = b1.parse().unwrap();
            let b2: u32 = b2.parse().unwrap();
            (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
        })
        .count();
    println!("Part 1: {num_contains}");

    // part 2
    let num_overlaps = input
        .lines()
        .filter(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();
            let a1: u32 = a1.parse().unwrap();
            let a2: u32 = a2.parse().unwrap();
            let b1: u32 = b1.parse().unwrap();
            let b2: u32 = b2.parse().unwrap();
            (a1 <= b2 && a2 >= b1) || (b1 <= a2 && b2 >= a1)
        })
        .count();
    println!("Part 2: {num_overlaps}");
}
