use regex::{Captures, Regex};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Claim {
	id: u32,
	left: u32,
	top: u32,
	width: u32,
	height: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = read_to_string("input.txt")?;
	let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
	let claims: Vec<Claim> = input
		.lines()
		.map(|line| caps_to_claim(re.captures(line).unwrap()))
		.collect();
	let squares_cut = squares_cut(&claims);
	println!("{:?}", part_1(&squares_cut));
	println!("{:?}", part_2(&claims, &squares_cut));
	Ok(())
}

fn squares_cut(claims: &Vec<Claim>) -> HashMap<(u32, u32), u32> {
	let mut seen = HashMap::<(u32, u32), u32>::new();
	for claim in claims {
		for x in 0..claim.width {
			for y in 0..claim.height {
				let location = (x + claim.left, y + claim.top);
				let count = seen.get(&location).unwrap_or(&0).to_owned();
				seen.insert(location, count + 1);
			}
		}
	}
	seen
}

fn caps_to_claim(caps: Captures) -> Claim {
	Claim {
		id: caps[1].parse().unwrap(),
		left: caps[2].parse().unwrap(),
		top: caps[3].parse().unwrap(),
		width: caps[4].parse().unwrap(),
		height: caps[5].parse().unwrap(),
	}
}

fn part_1(squares_cut: &HashMap<(u32, u32), u32>) -> usize {
	squares_cut.values().filter(|count| count > &&1).count()
}

fn part_2(claims: &Vec<Claim>, squares_cut: &HashMap<(u32, u32), u32>) -> u32 {
	'outer: for claim in claims {
		for x in 0..claim.width {
			for y in 0..claim.height {
				let location = (x + claim.left, y + claim.top);
				if let Some(count) = squares_cut.get(&location) {
					if count > &1u32 {
						continue 'outer;
					}
				}
			}
		}
		return claim.id;
	}
	0
}
