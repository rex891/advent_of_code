use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let container_sizes = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect_vec();

    let mut switches = [false; 20];
    let mut count = 0;
    let mut countainer_counts = [0; 20];

    loop {
        if 150
            == container_sizes
                .iter()
                .zip(switches)
                .filter_map(
                    |(container_size, switch)| if switch { Some(container_size) } else { None },
                )
                .sum::<u32>()
        {
            count += 1;
            let num_containers = switches.iter().filter(|&&bool_val| bool_val).count();
            countainer_counts[num_containers] += 1;
        }

        if !increment_array_bool(&mut switches) {
            break;
        }
    }
    println!("part 1: {:?}", count);
    println!("part 2: {:?}", countainer_counts);
}

fn increment_array_bool(input: &mut [bool]) -> bool {
    for item in input {
        if *item == false {
            *item = true;
            return true;
        } else {
            *item = false
        }
    }
    return false;
}
