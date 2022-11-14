use itertools::{max, min, Itertools};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut distances = HashMap::new();
    let mut cities = HashSet::new();

    // populate distance hashmap and cities set
    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        if let [first, _, second, _, val] = &tokens[..] {
            cities.insert(*first);
            cities.insert(*second);
            distances.insert((*first, *second), val.parse::<u32>().unwrap());
            distances.insert((*second, *first), val.parse::<u32>().unwrap());
        }
    }

    let sums = cities.iter().permutations(8).map(|perm| {
        perm.iter()
            .zip(perm.iter().skip(1))
            .map(|(a, b)| distances.get(&(**a, **b)).unwrap())
            .sum::<u32>()
    });

    println!("part 1: {}", min(sums.clone()).unwrap());
    println!("part 2: {}", max(sums).unwrap());
}
