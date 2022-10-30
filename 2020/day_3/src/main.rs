use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut east = 0;
    let mut trees = 0;
    for line in input.lines().skip(1) {
        east += 3;
        let val = line.chars().cycle().skip(east).next().unwrap();
        if val == '#' {
            trees += 1;
        }
    }
    println!("part1: {:?}", trees);

    let mut total = 1u64;
    for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut east = 0;
        let mut trees = 0;
        for line in input.lines().skip(down).step_by(down) {
            east += right;
            let val = line.chars().cycle().skip(east).next().unwrap();
            if val == '#' {
                trees += 1;
            }
        }

        total *= trees;
    }
    println!("part2: {:?}", total);
}
