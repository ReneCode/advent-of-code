use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;

use crate::util::io;

pub fn day19() {
    let lines = io::read_lines("./src/day19/19.data").unwrap();
    let all_lines = lines.join("\n");
    let groups = all_lines.split("\n\n").collect_vec();

    let patterns = groups[0].split(",").map(|l| l.trim()).collect_vec();
    let towls = groups[1].split("\n").collect_vec();

    part1(&patterns, &towls);

    let mut count_cache = HashMap::new();
    part2(&patterns, &towls, &mut count_cache);
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

fn part2(patterns: &[&str], towls: &[&str], count_cache: &mut HashMap<String, usize>) {
    let mut result = 0;
    for towl in towls {
        let count = count_solutions_to_build_towl(patterns, towl, count_cache);
        println!("Checking towl: {:?} / {:?}", towl, count);
        result += count;
    }
    println!("Day19 part 2: {:?}", result);
}

fn count_solutions_to_build_towl(
    pattern: &[&str],
    towl: &str,
    count_cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(count) = count_cache.get(towl) {
        return *count;
    }

    let mut count = 0;
    let fits = get_matching_patterns(pattern, towl);

    for fit in fits {
        let test_towl = &towl[fit.len()..];
        if test_towl.len() == 0 {
            count += 1;
        } else {
            let sub_count = count_solutions_to_build_towl(pattern, test_towl, count_cache);
            count += sub_count;
        }
    }
    count_cache.insert(towl.to_string(), count);

    count
}

fn can_build_towl(pattern: &[&str], towl: &str) -> bool {
    let fits = get_matching_patterns(pattern, towl);
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

fn get_matching_end_patterns(patterns: &[&str], towl: &str) -> Vec<String> {
    let mut result = Vec::new();
    for pattern in patterns {
        if towl.ends_with(pattern) {
            // result.push(pattern.to_string());
            result.push(pattern.to_string());
        }
    }
    result
}
