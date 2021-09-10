use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
	let input = read_to_string("input.txt")?;
	println!("part 1: {:?}", part_1(&input));
	println!("part 2: {:?}", part_2(&input));
	Ok(())
}

fn char_to_int(c: char) -> i32 {
	c.to_digit(10).unwrap() as i32
}

fn part_1(input: &str) -> i32 {
	let first_char = input.chars().next().unwrap().to_lowercase();

	// add first char to end of other chars
	let all = input.chars().chain(first_char);

	let mut sum = 0;
	let mut previous = -1;
	for current in all.map(|c| char_to_int(c)) {
		if current == previous {
			sum += current;
		}
		previous = current;
	}

	sum
}

fn part_2(input: &str) -> i32 {
	input
		.chars()
		.zip(input.chars().skip(input.len() / 2))
		.map(|(a, b)| (char_to_int(a), char_to_int(b)))
		.fold(0, |acc, (a, b)| if a == b { acc + a } else { acc })
		* 2
}
