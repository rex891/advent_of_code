use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // part 1
    let mut valid_passports = 0;
    let valid_fields = HashSet::from(["eyr", "hgt", "ecl", "hcl", "iyr", "pid", "byr"]);

    for passport in input.split("\n\n") {
        let fields: HashSet<_> = passport
            .split_ascii_whitespace()
            .map(|field| field.split_once(':').unwrap().0)
            .collect();

        if (&valid_fields - &fields).is_empty() {
            valid_passports += 1
        };
    }
    println!("part1: {:?}", valid_passports);

    // part 2
    valid_passports = 0;

    fn year_valid(o_yr: Option<&String>, lo: usize, hi: usize) -> bool {
        o_yr.and_then(|yr| match yr.len() {
            4 => Some(yr),
            _ => None,
        })
        .and_then(|yr| yr.parse().ok())
        .map(|year| lo <= year && year <= hi)
            == Some(true)
    }

    for passport in input.split("\n\n") {
        let fields: HashMap<String, String> = passport
            .split_ascii_whitespace()
            .map(|field| field.split_once(':').unwrap())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let hgt_valid = fields.get("hgt").and_then(|height| {
            let (val, unit) = height.split_at(height.len() - 2);
            val.parse().ok().map(|num| match unit {
                "cm" => 150 <= num && num <= 193,
                "in" => 59 <= num && num <= 76,
                _ => false,
            })
        }) == Some(true);

        let hcl_valid = fields.get("hcl").and_then(|hair_color| {
            hair_color
                .strip_prefix('#')
                .and_then(|hex_val| u32::from_str_radix(hex_val, 16).ok().map(|_| true))
        }) == Some(true);

        let valid_eyecolors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        let ecl_valid = fields
            .get("ecl")
            .map(|eye_color| valid_eyecolors.contains(&&eye_color[..]))
            == Some(true);

        let pid_valid = fields
            .get("pid")
            .map(|pid| pid.len() == 9 && pid.parse::<u32>().is_ok())
            == Some(true);

        if year_valid(fields.get("byr"), 1920, 2002)
            && (year_valid(fields.get("iyr"), 2010, 2020))
            && (year_valid(fields.get("eyr"), 2020, 2030))
            && (hgt_valid)
            && (hcl_valid)
            && (ecl_valid)
            && (pid_valid)
        {
            valid_passports += 1;
        }
    }

    println!("part2: {:?}", valid_passports);
}
