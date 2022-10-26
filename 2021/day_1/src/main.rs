use std::fs;
fn main() {
    let input: Vec<i32> = fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    println!("{:?}", count_deepers(&input));
    println!("{:?}", count_deepers2(&input));
}

fn count_deepers(numbers: &[i32]) -> usize {
    let mut count = 0;
    let mut it = numbers.iter();
    let mut prev = it.next().unwrap();
    for curr in it {
        if curr > prev {
            count += 1
        }
        prev = curr;
    }
    count
}

fn count_deepers2(numbers: &[i32]) -> usize {
    let mut count = 0;
    let mut it = numbers.iter();
    let mut first = it.next().unwrap();
    let mut second = it.next().unwrap();
    let mut third = it.next().unwrap();
    for fourth in it {
        if fourth > first {
            count += 1
        }
        first = second;
        second = third;
        third = fourth;
    }
    count
}
