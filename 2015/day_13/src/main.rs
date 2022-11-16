use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // parse data
    let input = fs::read_to_string("input.txt").unwrap();
    let mut rankings = HashMap::new();
    let mut guests = HashSet::new();
    for line in input.lines() {
        let tokens = line
            .strip_suffix('.')
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();
        if let &[person, _, direction, val, .., neighbor] = &tokens[..] {
            guests.insert(person);
            rankings.insert(
                (person, neighbor),
                val.parse::<i32>().unwrap() * (if direction == "lose" { -1 } else { 1 }),
            );
        }
    }

    // part 1
    let mut max_happiness = 0;
    for mut perm in guests.iter().permutations(8) {
        perm.push(perm.get(0).unwrap());
        let happiness = perm
            .iter()
            .zip(perm.iter().skip(1))
            .map(|(&&person, &&guest)| {
                rankings.get(&(person, guest)).unwrap() + rankings.get(&(guest, person)).unwrap()
            })
            .sum();

        max_happiness = max_happiness.max(happiness);
    }
    println!("part 1: {}", max_happiness);

    // part 2
    let mut max_happiness = 0;
    for mut perm in guests.iter().permutations(8) {
        perm.push(perm.get(0).unwrap());
        for i in 0..8 {
            let happiness = perm
                .iter()
                .zip(perm.iter().skip(1))
                .enumerate()
                .map(|(j, (&&person, &&guest))| {
                    if i == j {
                        return 0;
                    }
                    rankings.get(&(person, guest)).unwrap()
                        + rankings.get(&(guest, person)).unwrap()
                })
                .sum();
            max_happiness = max_happiness.max(happiness);
        }
    }
    println!("part 2: {}", max_happiness);
}
