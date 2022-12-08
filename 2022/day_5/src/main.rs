use regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    //build initial state
    let mut stacks: [Vec<char>; 9] = Default::default();
    loop {
        let s = lines.next().unwrap();

        // break out of building initial state
        if !s.contains('[') {
            lines.next();
            break;
        };

        for (i, c) in s.match_indices(char::is_alphabetic) {
            let column = (i - 1) / 4;
            let c = c.chars().next().unwrap();
            stacks[column].push(c);
        }
    }
    for col in 0..=8 {
        stacks[col].reverse()
    }

    // copy initial state for part 2
    let mut stacks2: [Vec<char>; 9] = stacks.clone();

    // build actions vector
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut actions = vec![];
    for line in lines {
        if let Some(cap) = re.captures(line) {
            let num_crates = cap[1].parse::<usize>().unwrap();
            let col_from = cap[2].parse::<usize>().unwrap() - 1;
            let col_to = cap[3].parse::<usize>().unwrap() - 1;
            actions.push((col_from, col_to, num_crates));
        }
    }

    // part1
    for &(col_from, col_to, num_crates) in &actions {
        for _ in 0..num_crates {
            let cr = stacks[col_from].pop().unwrap();
            stacks[col_to].push(cr);
        }
    }

    // get top crates
    let s: String = (0..=8).map(|col| stacks[col].pop().unwrap()).collect();
    println!("Part 1: {s}");

    // part2
    let mut crate_mover_9001 = vec![];
    for &(col_from, col_to, num_crates) in &actions {
        for _ in 0..num_crates {
            let cr = stacks2[col_from].pop().unwrap();
            crate_mover_9001.push(cr);
        }
        for _ in 0..num_crates {
            stacks2[col_to].push(crate_mover_9001.pop().unwrap());
        }
    }
    // get top crates
    let s: String = (0..=8).map(|col| stacks2[col].pop().unwrap()).collect();
    println!("Part 2: {s}");
}
