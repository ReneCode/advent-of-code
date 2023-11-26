// day04

use itertools::Itertools;

use crate::util::io;

// ignore "cid"
const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn day04() {
    let lines = io::read_lines("04.data").unwrap();
    let all_lines = lines.iter().join("\n");
    let passports: Vec<&str> = all_lines.split("\n\n").collect();

    let mut count_valid = 0;
    for passport in passports {
        let lines = passport.split("\n").collect_vec();
        let keys = get_keys(&lines);
        if all_required_keys_there(&keys) {
            count_valid += 1;
        }
        // println!("passport lines: {:?}", lines);
    }

    println!("result A: {count_valid}");
}

fn all_required_keys_there(keys: &[&str]) -> bool {
    for key in REQUIRED_KEYS {
        if !keys.contains(&key) {
            return false;
        }
    }
    true
}

fn get_keys<'a>(lines: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in lines {
        let kv_pairs = line.split(' ').map(|l| l.trim()).collect_vec();
        for pair in kv_pairs {
            let token = pair.split(':').map(|t| t.trim()).collect_vec();
            let key = token.get(0).unwrap();
            result.push(*key);
        }
    }

    result
}
