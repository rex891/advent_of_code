use std::collections::HashMap;

use regex::Regex;
fn main() {
    let re = Regex::new(
        r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)",
    )
    .unwrap();
    let input = "Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2\n\
                       Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9\n\
                       Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1\n\
                       Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8";
    let x = &re
        .captures_iter(input)
        .map(|caps| {
            let name = &caps[1][..];
            let capacity = caps[2].parse::<i32>().unwrap();
            let durability = caps[3].parse::<i32>().unwrap();
            let flavor = caps[4].parse::<i32>().unwrap();
            let texture = caps[5].parse::<i32>().unwrap();
            let calories = caps[6].parse::<i32>().unwrap();
            (
                name,
                CookieProperties {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            )
        })
        .collect::<HashMap<_, _>>();
}

struct CookieProperties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}
