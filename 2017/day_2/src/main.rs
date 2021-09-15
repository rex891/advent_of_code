use std::fs::read_to_string;
use std::iter::Iterator;

fn main() -> std::io::Result<()> {
	let input = read_to_string("input.txt")?;
	println!("{:?}", part_1(&input));
	println!("{:?}", part_2(&input));
	Ok(())
}

fn part_1(input: &str) -> i32 {
	let mut sum = 0;

	for l in input.lines() {
		let mut split_line = l.split_whitespace();

		// init min and max with first value
		let first = split_line.next().unwrap().parse::<i32>().unwrap();
		let mut min = first;
		let mut max = first;

		// compare with rest of values
		for val in split_line {
			let x = val.parse::<i32>().unwrap();
			if x < min {
				min = x;
			}
			if x > max {
				max = x;
			}
		}

		// add difference between line extremes
		sum += max - min;
	}
	sum
}

fn part_2(input: &str) -> i32 {
	let mut sum = 0;

	for l in input.lines() {
		let vals: Vec<i32> = l
			.split_whitespace()
			.map(|val| val.parse::<i32>().unwrap())
			.collect();

		'outer: for (i, val) in vals.iter().enumerate() {
			for other_val in vals.iter().skip(i + 1) {
				if val % other_val == 0 {
					sum += val / other_val;
					break 'outer;
				}
				if other_val % val == 0 {
					sum += other_val / val;
					break 'outer;
				}
			}
		}
	}
	sum
}
