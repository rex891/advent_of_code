use regex::Regex;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut count = 0;
    for mut line in input.lines() {
        line = line.strip_prefix('"').unwrap();
        line = line.strip_suffix('"').unwrap();
        let re = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
        println!("{:?}", re.find_iter(line).count());

        let line_tot = re.find_iter(line).count() * 3
            + line.matches(r"\\").count()
            + line.matches("\\\"").count();
        // println!(" {}", line_tot);
        count += 2 + line_tot;
    }
    println!("{}", count);
    // dbg!(input);
}
