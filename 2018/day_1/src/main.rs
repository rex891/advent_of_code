use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input: Vec<_> = read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let plus = &line[0..1] == "+";
            let x = &line[1..].to_owned();
            let val: i32 = x.parse().unwrap();
            (plus, val)
        })
        .collect();
    println!("Part1: {:?}", part_1(&input));
    println!("Part2: {:?}", part_2(&input));

    Ok(())
}

fn part_1(input: &Vec<(bool, i32)>) -> i32 {
    let mut sum = 0;
    for (plus, val) in input.iter() {
        if *plus {
            sum += val;
        } else {
            sum -= val;
        }
    }
    sum
}

fn part_2(input: &Vec<(bool, i32)>) -> i32 {
    let mut seen = std::collections::HashSet::new();
    seen.insert(0);

    let mut curr_val = 0;

    for (plus, val) in input.iter().cycle() {
        if *plus {
            curr_val += val;
        } else {
            curr_val -= val;
        }
        if seen.contains(&curr_val) {
            return curr_val;
        }
        seen.insert(curr_val);
    }
    -1
}
