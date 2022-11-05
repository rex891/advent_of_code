use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    // part 1
    println!(
        "part 1: {}",
        input
            .chars()
            .fold(0, |acc, item| if item == '(' { acc + 1 } else { acc - 1 })
    );

    // part 2
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        floor += if c == '(' { 1 } else { -1 };
        if floor < 0 {
            println!("part 2: {}", i + 1);
            break;
        }
    }

    Ok(())
}
