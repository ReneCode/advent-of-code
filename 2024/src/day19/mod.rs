use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

use crate::util::io;

pub fn day19() {
    let lines = io::read_lines("./src/day19/19.data").unwrap();
    let all_lines = lines.join("\n");
    let groups = all_lines.split("\n\n").collect_vec();

    let patterns = groups[0].split(",").map(|l| l.trim()).collect_vec();
    let towls = groups[1].split("\n").collect_vec();

    part1(&patterns, &towls);

    let result = 0;
}

fn part1(patterns: &[&str], towls: &[&str]) {
    let mut result = 0;
    for towl in towls {
        // println!("Checking towl: {:?}", towl);
        if can_build_towl(patterns, towl) {
            result += 1;
        }
    }
    println!("Day19 part 1: {:?}", result);
}

fn can_build_towl(pattern: &[&str], towl: &str) -> bool {
    let mut fits = get_matching_patterns(pattern, towl);
    let mut checks = HashSet::new();
    for fit in fits {
        checks.insert(fit);
    }

    while checks.len() > 0 {
        let mut new_checks = HashSet::new();
        for check in checks {
            let test_towl = &towl[check.len()..];
            let fits = get_matching_patterns(pattern, test_towl);
            for fit in fits {
                let mut concat = check.clone();
                concat.push_str(&fit);
                if concat.len() == towl.len() {
                    return true;
                }
                new_checks.insert(concat);
            }
        }
        checks = new_checks;
    }

    false
}

fn get_matching_patterns(patterns: &[&str], towl: &str) -> Vec<String> {
    let mut result = Vec::new();
    for pattern in patterns {
        if towl.starts_with(pattern) {
            // result.push(pattern.to_string());
            result.push(pattern.to_string());
        }
    }
    result
}
