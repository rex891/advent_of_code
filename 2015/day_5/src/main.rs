use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let count = input
        .lines()
        .filter(|c| contains_3_vowels(c))
        .filter(|c| contains_double_letter(c))
        .filter(|c| doesnt_contain_bad(c))
        .collect::<Vec<_>>()
        .len();
    println!("part 1: {count}");

    let count = input
        .lines()
        .filter(|c| pair_of_double_letters_no_overlap(c))
        .filter(|c| pair_with_one_between(c))
        .collect::<Vec<_>>()
        .len();

    println!("part 2: {count}");
}

fn contains_3_vowels(word: &str) -> bool {
    // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    let vowels: HashSet<char> = "aeiou".chars().collect();
    word.chars().filter(|c| vowels.contains(c)).count() >= 3
}
fn contains_double_letter(word: &str) -> bool {
    // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    let mut prev_c = '-';
    for c in word.chars() {
        if c == prev_c {
            return true;
        } else {
            prev_c = c
        }
    }
    false
}

fn doesnt_contain_bad(word: &str) -> bool {
    // It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
    let bad: HashSet<&str> = ["ab", "cd", "pq", "xy"].into_iter().collect();
    let mut prev_c = '-';
    for c in word.chars() {
        if bad.contains::<str>(format!("{prev_c}{c}").as_ref()) {
            return false;
        } else {
            prev_c = c
        }
    }
    true
}

fn pair_of_double_letters_no_overlap(word: &str) -> bool {
    // It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    for (idx1, a) in (0..).map_while(|i| word.get(i..i + 2)).enumerate() {
        for b in ((idx1 + 2)..).map_while(|i| word.get(i..i + 2)) {
            if a == b {
                return true;
            }
        }
    }
    false
}

fn pair_with_one_between(word: &str) -> bool {
    // It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
    for (a, b) in word.chars().zip(word.chars().skip(2)) {
        if a == b {
            return true;
        }
    }
    false
}
