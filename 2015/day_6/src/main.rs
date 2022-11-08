use regex::Regex;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut light_on = [[false; 1000]; 1000];
    let mut brightness = [[0; 1000]; 1000];

    let re = Regex::new(r"(\D*) (\d*),(\d*)\D*(\d*),(\d*)").unwrap();
    for line in input.lines() {
        let caps = &re.captures(line).unwrap();
        let cmd = &caps[1];
        let x0: usize = caps[2].parse().unwrap();
        let y0: usize = caps[3].parse().unwrap();
        let x1: usize = caps[4].parse().unwrap();
        let y1: usize = caps[5].parse().unwrap();
        for x in x0..=x1 {
            for y in y0..=y1 {
                match cmd {
                    "turn off" => {
                        light_on[x][y] = false;
                        if brightness[x][y] != 0 {
                            brightness[x][y] -= 1
                        }
                    }
                    "turn on" => {
                        light_on[x][y] = true;
                        brightness[x][y] += 1
                    }
                    "toggle" => {
                        light_on[x][y] = !light_on[x][y];
                        brightness[x][y] += 2
                    }
                    _ => {}
                }
            }
        }
    }
    let mut count = 0;
    let mut total_bright = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if light_on[x][y] == true {
                count += 1
            };
            total_bright += brightness[x][y];
        }
    }
    println!("part1: {}", count);
    println!("part2: {}", total_bright);
}
