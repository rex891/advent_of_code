use std::fs;
fn main() {
    let input: Vec<u32> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    'outer: for num1 in &input {
        for num2 in &input {
            if num1 + num2 == 2020 {
                println!("{}", num1 * num2);
                break 'outer;
            }
        }
    }

    'outer: for num1 in &input {
        for num2 in &input {
            for num3 in &input {
                if num1 + num2 + num3 == 2020 {
                    println!("{}", num1 * num2 * num3);
                    break 'outer;
                }
            }
        }
    }
}
