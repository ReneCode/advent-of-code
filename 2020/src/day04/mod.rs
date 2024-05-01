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
            // println!("passport lines: {:?}", lines);

            let ok = match part {
                'A' => all_required_keys_there(&key_value_pairs),
                'B' => all_required_keys_there(&key_value_pairs) && all_values_ok(&key_value_pairs),
                _ => false,
            };
            if ok {
                count_valid += 1
            }
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
            "byr" => {
                let year = value.parse::<i32>().unwrap();
                year >= 1920 && year <= 2002
            }
            "iyr" => {
                let year = value.parse::<i32>().unwrap();
                year >= 2010 && year <= 2020
            }
            "eyr" => {
                let year = value.parse::<i32>().unwrap();
                year >= 2020 && year <= 2030
            }
            "hgt" => {
                if value.len() < 3 {
                    return false;
                }
                let unit = &value[value.len() - 2..];
                let height = value[..value.len() - 2].parse::<i32>().unwrap();
                match unit {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false,
                }
            }
            "hcl" => {
                for (i, c) in value.chars().enumerate() {
                    if i == 0 {
                        if c != '#' {
                            return false;
                        }
                    } else {
                        if !c.is_ascii_hexdigit() {
                            return false;
                        }
                    }
                }
                true
            }
            "ecl" => {
                let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                colors.contains(&value)
            }
            "pid" => value.len() == 9 && value.chars().all(char::is_numeric),
            "cid" => true,

            _ => false,
        };
        // println!("key: {:?}, value: {:?}, ok: {:?}", key, value, ok);
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
