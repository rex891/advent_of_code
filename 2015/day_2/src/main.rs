use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let mut total_wrapping: u32 = 0;
    let mut total_ribbon: u32 = 0;

    for dims in input.lines().map(|line| {
        line.split('x')
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>()
    }) {
        if let &[length, width, height] = &dims[..] {
            // wrapping
            let sides_area: [u32; 3] = [length * width, length * height, width * height];
            total_wrapping += 2 * sides_area.iter().sum::<u32>() + sides_area.iter().min().unwrap();

            // ribbon
            let mut sides_length: [u32; 3] = [length, width, height];
            sides_length.sort();
            let perimeter = 2 * sides_length[..2].iter().sum::<u32>();
            let bow = sides_length.iter().product::<u32>();
            total_ribbon += perimeter + bow;
        }
    }
    println!("part 1: {total_wrapping}");
    println!("part 2: {total_ribbon}");

    Ok(())
}
