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
        sum += a * b;
    }

    println!("Part1: {}", sum);

    part2(&line);
}

fn part2(line: &str) {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    let mut sum: i64 = 0;
    let mut do_it = true;

    for tok in re.captures_iter(line) {
        let full_cmd = &tok[0];
        let cmd = &full_cmd[0..=2];
        match cmd {
            "don" => {
                do_it = false;
            }
            "do(" => {
                do_it = true;
            }
            "mul" => {
                if do_it {
                    sum += calc_mul(&tok[0]);
                }
            }
            _ => {
                panic!("Unknown cmd: {}", cmd);
            }
        }
    }
    println!("Part2: {}", sum);
}

fn calc_mul(cmd: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let tok = re.captures(cmd).unwrap();
    let a: i64 = tok.get(1).unwrap().as_str().parse().unwrap();
    let b: i64 = tok.get(2).unwrap().as_str().parse().unwrap();
    a * b
}
