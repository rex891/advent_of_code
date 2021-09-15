use ::std::fs::read_to_string;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
	let input: Vec<Vec<char>> = read_to_string("input.txt")?
		.lines()
		.map(|line| line.chars().collect())
		.collect();

	println!("{:?}", part_1(&input));
	println!("{:?}", part_2(&input));

	Ok(())
}

fn part_1(input: &Vec<Vec<char>>) -> i32 {
	let mut twos = 0;
	let mut threes = 0;
	for line in input {
		let mut seen = HashMap::new();

		// count letters in line
		for c in line {
			let count = seen.get(c).unwrap_or(&0u8).to_owned();
			seen.insert(*c, count + 1);
		}

		let x: Vec<_> = seen.values().collect();
		if x.contains(&&2u8) {
			twos += 1;
		}

		if x.contains(&&3u8) {
			threes += 1;
		}
	}
	twos * threes
}

fn part_2(input: &Vec<Vec<char>>) -> String {
	for (i, id_1) in input.iter().enumerate() {
		for j in i..input.len() {
			let id_2 = &input[j];
			let mut mismatches = vec![];
			for (k, (c1, c2)) in id_1.iter().zip(id_2).enumerate() {
				if c1 != c2 {
					mismatches.push(k);
				}
			}
			if mismatches.len() == 1 {
				let mut code = id_1.to_owned();
				code.remove(mismatches[0]);
				return code.into_iter().collect();
			}
		}
	}
	String::from("")
}
