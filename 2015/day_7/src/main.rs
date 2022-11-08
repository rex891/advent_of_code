use std::{collections::HashMap, fs};

fn get_val(name_or_val: &str, values: &HashMap<String, u16>) -> Option<u16> {
    name_or_val
        .parse()
        .ok()
        .or(values.get(name_or_val).copied())
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut values: HashMap<String, u16> = HashMap::new();

    let inputs = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(inp, name)| (name.to_string(), inp.split(" ").collect::<Vec<&str>>()))
        .collect::<HashMap<_, _>>();

    loop {
        for (name, inp) in &inputs {
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
            println!("part 1: {}", val);
            break;
        }
    }
}
