use serde_json::Value::{self, *};
use std::fs;
fn main() {
    let input = fs::read_to_string("input.json").unwrap();
    let re = regex::Regex::new(r"([-]?\d+)").unwrap();
    let v: i32 = re
        .captures_iter(&input)
        .map(|cap| cap.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .sum();
    println!("part 1: {:?}", v);

    let input_json: Value = serde_json::from_str(&input).unwrap();
    println!("part 2: {:?}", get_sum(&input_json));
}

fn get_sum(node: &Value) -> i64 {
    match node {
        Number(num) => num.as_i64().unwrap(),
        Array(vec) => vec.iter().map(|v| get_sum(v)).sum(),

        // if any object value is "red" the whole object is and children val is 0
        Object(obj) => {
            if obj.values().any(|v| {
                if let String(str_val) = v {
                    str_val == "red"
                } else {
                    false
                }
            }) {
                0
            } else {
                obj.values().map(|v| get_sum(v)).sum()
            }
        }
        _ => 0,
    }
}
