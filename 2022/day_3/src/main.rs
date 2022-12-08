use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // part 1
    let mut letters = vec![];
    'a: for line in input.lines() {
        let (a, b) = line.split_at(line.len() / 2);
        for c in a.chars() {
            if b.contains(c) {
                letters.push(c);
                continue 'a;
            }
        }
    }
    let score: u32 = letters
        .iter()
        .map(|&l| l as u32)
        .map(|l| if l >= 65 && l <= 96 { l - 38 } else { l - 96 })
        .sum();
    println!("Part 1: {score}");

    // part 2:
    let mut lines = input.lines();
    let mut letters = vec![];
    loop {
        if let (Some(s1), Some(s2), Some(s3)) = (lines.next(), lines.next(), lines.next()) {
            for c in s1.chars() {
                if s2.contains(c) && s3.contains(c) {
                    letters.push(c);
                    break;
                }
            }
        } else {
            break;
        };
    }
    let score: u32 = letters
        .iter()
        .map(|&l| l as u32)
        .map(|l| if l >= 65 && l <= 96 { l - 38 } else { l - 96 })
        .sum();
    println!("Part 2: {score}");
}
