use std::fmt::format;

use itertools::Itertools;

fn main() {
    let mut input = "1113222113".to_string();
    for i in 1..=50 {
        input = get_next_num(&input);
        if i == 40 {
            println!("part 1: {}", input.len())
        }
        if i == 50 {
            println!("part 2: {}", input.len())
        }
    }
}

fn get_next_num(input: &str) -> String {
    input
        .chars()
        .dedup_with_count()
        .map(|(count, ch)| count.to_string() + &ch.to_string())
        .collect::<String>()
}
