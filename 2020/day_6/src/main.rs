use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for form in input.split("\n\n") {
        let mut set = HashSet::new();
        for line in form.lines() {
            set.extend(line.chars());
        }

        sum += set.len();
    }
    println!("part 1: {}", sum);

    sum = 0;
    for form in input.split("\n\n") {
        let mut char_sets = form
            .lines()
            .map(str::chars)
            .map(|x| x.collect::<HashSet<char>>());

        let mut first = char_sets.next().unwrap();
        for char_set in char_sets {
            first = first.intersection(&char_set).cloned().collect();
        }

        sum += first.len();
    }
    println!("part 2: {}", sum);
}
