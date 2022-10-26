use regex::Regex;
use std::error::Error;
use std::fs;
fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;
    let input = fs::read_to_string("input.txt")?;

    let passwords: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            re.captures(line)
                .unwrap()
                .iter()
                .map(|m| m.unwrap().as_str())
                .collect()
        })
        .collect();

    let mut i = 0;
    for pwd in &passwords {
        let lo = pwd[1].parse::<usize>()?;
        let hi = pwd[2].parse::<usize>()?;
        let letter = pwd[3].parse::<char>()?;
        let password = pwd[4].parse::<String>()?;
        let count = password.chars().filter(|ch| ch == &letter).count();

        if lo <= count && count <= hi {
            i += 1
        }
    }
    println!("part1: {:?}", i);

    let mut i = 0;
    for pwd in &passwords {
        let pos1 = pwd[1].parse::<usize>()?;
        let pos2 = pwd[2].parse::<usize>()?;
        let letter = pwd[3].as_bytes()[0];
        let password = pwd[4].parse::<String>()?;
        let pw_bytes = password.as_bytes();

        if (pw_bytes[pos1 - 1] == letter && pw_bytes[pos2 - 1] != letter)
            || (pw_bytes[pos1 - 1] != letter && pw_bytes[pos2 - 1] == letter)
        {
            i += 1
        }
    }
    println!("part2: {:?}", i);

    Ok(())
}
