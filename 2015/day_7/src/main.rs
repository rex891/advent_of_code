use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let inputs = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(inp, name)| (name.to_string(), inp.split(" ").collect::<Vec<&str>>()))
        .collect::<HashMap<_, _>>();

    // part 1
    let part_1_val = get_a_signal(&inputs, HashMap::new());
    println!("Part 1: {}", part_1_val);

    // part 2
    let mut vals2 = HashMap::new();
    vals2.insert("b".to_string(), part_1_val);
    let part_2_val = get_a_signal(&inputs, vals2);
    println!("Part 2: {}", part_2_val);
}

fn get_val(name_or_val: &str, values: &HashMap<String, u16>) -> Option<u16> {
    name_or_val
        .parse()
        .ok()
        .or(values.get(name_or_val).copied())
}

fn get_a_signal(inputs: &HashMap<String, Vec<&str>>, mut values: HashMap<String, u16>) -> u16 {
    loop {
        for (name, inp) in inputs {
            if values.get(name).is_some() {
                continue;
            }
            match &inp[..] {
                [a] => {
                    if let Some(val) = get_val(a, &values) {
                        values.insert(name.clone(), val);
                    }
                }
                ["NOT", a] => {
                    if let Some(val) = get_val(a, &values) {
                        values.insert(name.clone(), !val);
                    }
                }
                [a, "AND", b] => {
                    if let (Some(v1), Some(v2)) = (get_val(a, &values), get_val(b, &values)) {
                        values.insert(name.clone(), v1 & v2);
                    }
                }
                [a, "OR", b] => {
                    if let (Some(v1), Some(v2)) = (get_val(a, &values), get_val(b, &values)) {
                        values.insert(name.clone(), v1 | v2);
                    }
                }
                [a, "RSHIFT", b] => {
                    let shift_amount = b.parse::<i32>().unwrap();
                    if let Some(val) = get_val(a, &values) {
                        values.insert(name.clone(), val >> shift_amount);
                    }
                }
                [a, "LSHIFT", b] => {
                    let shift_amount = b.parse::<i32>().unwrap();
                    if let Some(val) = get_val(a, &values) {
                        values.insert(name.clone(), val << shift_amount);
                    }
                }

                [..] => {
                    println!("OTHER")
                }
            }
        }
        if let Some(val) = get_val("a", &values) {
            return val;
        }
    }
}
