use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input");

    // part 1
    let compounds_detected = [
        "children: 3",
        "cats: 7",
        "samoyeds: 2",
        "pomeranians: 3",
        "akitas: 0",
        "vizslas: 0",
        "goldfish: 5",
        "trees: 3",
        "cars: 2",
        "perfumes: 1",
    ];
    let compounds_per_sue = input.lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .split(',')
            .map(|x| x.trim())
    });

    for (i, mut compounds) in compounds_per_sue.clone().enumerate() {
        if compounds.all(|compound| compounds_detected.contains(&compound)) {
            println!("part 1: {:?}", i + 1);
        }
    }

    // part 2
    let compound_count: HashMap<&str, u32> = compounds_detected
        .map(|v| {
            let (thing, val) = v.split_once(": ").unwrap();
            let val: u32 = val.parse().unwrap();
            (thing, val)
        })
        .into_iter()
        .collect();

    for (i, mut compounds) in compounds_per_sue.enumerate() {
        if compounds.all(|compound| match compound.split_once(": ") {
            Some((a @ ("cats" | "trees"), num)) => {
                num.parse::<u32>().unwrap() > *compound_count.get(a).unwrap()
            }
            Some((a @ ("pomeranians" | "goldfish"), num)) => {
                num.parse::<u32>().unwrap() < *compound_count.get(a).unwrap()
            }
            _ => compounds_detected.contains(&compound),
        }) {
            println!("part 2: {:?}", i + 1);
        }
    }
}
