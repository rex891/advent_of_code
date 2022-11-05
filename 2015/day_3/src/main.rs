use std::{collections::HashMap, fs};

fn main() {
    // part 1
    let input = fs::read_to_string("input.txt").unwrap();

    let mut visited_locations: Locations = HashMap::new();
    add_locations(&mut visited_locations, &input);

    println!("part 1: {}", visited_locations.len());

    // part 2
    let (santa_route, robosanta_route) = input.chars().enumerate().fold(
        (String::new(), String::new()),
        |(s_route, r_route), (i, c)| {
            if i % 2 == 0 {
                (s_route + &c.to_string(), r_route)
            } else {
                (s_route, r_route + &c.to_string())
            }
        },
    );
    let mut visited_locations: Locations = HashMap::new();
    add_locations(&mut visited_locations, &santa_route);
    add_locations(&mut visited_locations, &robosanta_route);
    println!("part 2: {}", visited_locations.len());
}

type Locations = HashMap<(i32, i32), bool>;

fn add_locations<'a>(visited_locations: &'a mut Locations, route: &str) -> &'a Locations {
    let mut current_location = (0, 0);
    visited_locations.insert(current_location, true);
    for c in route.chars() {
        match c {
            '^' => current_location = (current_location.0, current_location.1 + 1),
            '>' => current_location = (current_location.0 + 1, current_location.1),
            'v' => current_location = (current_location.0, current_location.1 - 1),
            '<' => current_location = (current_location.0 - 1, current_location.1),
            _ => {}
        }
        visited_locations.insert(current_location, true);
    }
    visited_locations
}
