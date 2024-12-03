use crate::util::io;

use regex::Regex;

pub fn day03() {
    let lines = io::read_lines("./src/day03/03.data").unwrap();
    let line = lines.join("");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for tok in re.captures_iter(line.as_str()) {
        let a: i64 = tok.get(1).unwrap().as_str().parse().unwrap();
        let b: i64 = tok.get(2).unwrap().as_str().parse().unwrap();
        sum += (a * b);
    }

    println!("Part1: {}", sum);
}
