use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let bag_rules = input
        .lines()
        .map(|line| {
            if let Some((container, items)) = line.split_once(" contain ") {
                let container = container.rsplit_once(" ").unwrap().0.to_string();

                let items = match items {
                    "no other bags." => None,
                    items => {
                        let items: Vec<Portion> = items
                            .split(", ")
                            .map(|it| {
                                let (number, color) =
                                    it.rsplit_once(" ").unwrap().0.split_once(" ").unwrap();
                                Portion {
                                    number: number.parse::<u32>().unwrap(),
                                    color,
                                }
                            })
                            .collect();

                        Some(items)
                    }
                };
                (container, items)
            } else {
                ("".to_string(), None)
            }
        })
        .collect::<BagRules>();

    let num_bags = bag_rules
        .keys()
        .map(|color| holds("shiny gold", color, &bag_rules))
        .filter(|&val| val)
        .count();

    println!("part 1: {num_bags}");

    println!(
        "part 2: {}",
        get_bag_contents_count("shiny gold", &bag_rules),
    );

    Ok(())
}

#[derive(Debug)]
struct Portion<'a> {
    number: u32,
    color: &'a str,
}

type BagRules<'a> = HashMap<String, Option<Vec<Portion<'a>>>>;

fn holds(find_color: &str, curr_bag_color: &str, bag_rules: &BagRules) -> bool {
    match bag_rules.get(curr_bag_color) {
        Some(Some(portions)) => {
            let mut inner_colors = portions.iter().map(|portion| portion.color);
            let mut y = inner_colors.clone();
            inner_colors.any(|col| col == find_color)
                || y.any(|col| holds(find_color, col, bag_rules))
        }
        _ => false,
    }
}

fn get_bag_contents_count(color: &str, bag_rules: &BagRules) -> u32 {
    match &bag_rules[color] {
        None => 1,
        Some(portions) => portions.iter().fold(0, |acc, Portion { number, color }| {
            println!("{acc}, {number}, {color}");
            acc + 1 + number * get_bag_contents_count(color, bag_rules)
        }),
    }
}
