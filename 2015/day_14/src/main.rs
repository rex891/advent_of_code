use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let time = 2503;

    // part 1
    for line in input.lines() {
        let vals = line.split(' ').collect::<Vec<_>>();
        let speed = vals[3].parse::<u32>().unwrap();
        let fly_duration = vals[6].parse::<u32>().unwrap();
        let rest_duration = vals[13].parse::<u32>().unwrap();

        let cycle_duration = fly_duration + rest_duration;
        let total_cycles = time / cycle_duration;
        let remaining_time = time % cycle_duration;

        let total_on = total_cycles * fly_duration + remaining_time.min(fly_duration);
        let distance = speed * total_on;

        println!("{:?}", distance);
    }

    // part 2
    //build array for each reindeer with distance at each time click
    let (reindeer, distances): (Vec<&str>, Vec<Vec<u32>>) = input
        .lines()
        .map(|line| {
            let vals = line.split(' ').collect::<Vec<_>>();
            let name = vals[0];
            let speed = vals[3].parse::<u32>().unwrap();
            let fly_duration = vals[6].parse::<u32>().unwrap();
            let rest_duration = vals[13].parse::<u32>().unwrap();

            let mut distances = Vec::with_capacity(2503);
            let mut is_flying = true;
            let mut time_left = fly_duration;
            let mut curr_distance = 0;

            for _ in 0..2503 {
                if is_flying {
                    curr_distance += speed;
                }
                distances.push(curr_distance);
                time_left -= 1;
                if time_left == 0 {
                    is_flying = !is_flying;
                    time_left = if is_flying {
                        fly_duration
                    } else {
                        rest_duration
                    }
                }
            }
            (name, distances)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();

    let mut scores = HashMap::new();

    for i in 0..2503 {
        //pair reindeer and distance or reindeer at each time click
        let mut reindeer_distances = reindeer
            .iter()
            .zip(distances.iter().map(|a| a[i]))
            .collect::<Vec<_>>();
        reindeer_distances.sort_by_key(|v| v.1);

        // give some points to the leaders
        if let Some((first_place_reindeer, first_place_distance)) = reindeer_distances.pop() {
            *scores.entry(first_place_reindeer).or_insert(0) += 1;
            while let Some((reindeer, distance)) = reindeer_distances.pop() {
                if distance == first_place_distance {
                    *scores.entry(reindeer).or_insert(0) += 1;
                } else {
                    break;
                }
            }
        }
    }
    dbg!(&scores);
}
