use regex::Regex;
use std::fs::read_to_string;

fn main() {
	let input = read_to_string("input.txt");
	let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
}
