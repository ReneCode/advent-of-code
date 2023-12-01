// day04

use itertools::Itertools;

use crate::util::io;

// ignore "cid"
const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn day04() {
    let lines = io::read_lines("04.data").unwrap();
    let all_lines = lines.iter().join("\n");
    let passports: Vec<&str> = all_lines.split("\n\n").collect();

    for part in ['A', 'B'] {
        let mut count_valid = 0;
        for passport in passports.iter() {
            let lines = passport.split("\n").collect_vec();
            let key_value_pairs = get_key_value_pairs(&lines);

            let ok = match part {
                'A' => all_required_keys_there(&key_value_pairs),
                'B' => all_required_keys_there(&key_value_pairs) && all_values_ok(&key_value_pairs),
                _ => false,
            };
            if ok {
                count_valid += 1
            }
            // println!("passport lines: {:?}", lines);
        }

        println!("result {part}: {count_valid}");
    }
}

fn all_required_keys_there(key_value_pairs: &[(&str, &str)]) -> bool {
    for key in REQUIRED_KEYS {
        if let None = key_value_pairs.iter().find(|(k, v)| *k == key) {
            return false;
        }
    }
    true
}

fn all_values_ok(key_value_pairs: &[(&str, &str)]) -> bool {
    for (key, value) in key_value_pairs {
        let ok = match (*key) {
            "byr" => true,
            _ => true,
        };
        if !ok {
            return false;
        }
    }
    true
}

fn get_key_value_pairs<'a>(lines: &'a Vec<&'a str>) -> Vec<(&'a str, &str)> {
    let mut result: Vec<(&str, &str)> = Vec::new();
    for line in lines {
        let kv_pairs = line.split(' ').map(|l| l.trim()).collect_vec();
        for pair in kv_pairs {
            let token = pair.split(':').map(|t| t.trim()).collect_vec();
            let key = token.get(0).unwrap();
            let value = token.get(1).unwrap();
            result.push((*key, *value));
        }
    }

    result
}
