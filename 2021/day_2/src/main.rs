use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let pairs: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .collect();

    println!("{}", part1(&pairs));
    println!("{}", part2(&pairs));
}

fn part1(pairs: &Vec<(&str, &str)>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;

    for pair in pairs {
        let direction = pair.0;
        let distance = pair.1.parse::<i32>().unwrap();

        match direction {
            "up" => depth -= distance,
            "down" => depth += distance,
            "forward" => horiz += distance,
            _ => continue,
        }
    }
    horiz * depth
}

fn part2(pairs: &Vec<(&str, &str)>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for pair in pairs {
        let direction = pair.0;
        let change = pair.1.parse::<i32>().unwrap();

        match direction {
            "up" => aim -= change,
            "down" => aim += change,
            "forward" => {
                horiz += change;
                depth += aim * change;
            }
            _ => continue,
        }
    }
    horiz * depth
}
