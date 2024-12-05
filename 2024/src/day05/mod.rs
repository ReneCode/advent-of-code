use std::collections::HashSet;

use crate::util::io;
use itertools::Itertools;

struct Rule {
    x: i64,
    y: i64,
}

impl Rule {
    fn is_valid(&self, a: i64, b: i64) -> bool {
        (a == self.x && b == self.y) || (a == self.y && b == self.x)
    }
}

struct Update {
    pages: Vec<i64>,
}

impl Update {
    fn middle_page(&self) -> i64 {
        let len = self.pages.len();
        let middle = len / 2;
        self.pages.iter().nth(middle).unwrap().clone()
    }
}

pub fn day05() {
    let lines = io::read_lines("./src/day05/05.data").unwrap();

    let all_lines = lines.join("\n");
    let groups = all_lines.split("\n\n").collect_vec();

    let rules = groups[0]
        .split('\n')
        .map(|t| {
            let parts = t.split("|").collect_vec();
            Rule {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        })
        .collect_vec();

    let updates = groups[1]
        .split('\n')
        .map(|t| Update {
            pages: t.split(",").map(|n| n.parse().unwrap()).collect_vec(),
        })
        .collect_vec();

    part1(&rules, &updates);
    part2(&rules, &updates);
}

fn part1(rules: &Vec<Rule>, updates: &Vec<Update>) {
    let mut sum = 0;
    for update in updates {
        if is_update_ok(update, rules) {
            sum += update.middle_page();
        }
    }
    println!("Day 05, part 1: {}", sum);
}

fn part2(rules: &Vec<Rule>, updates: &Vec<Update>) {
    let mut sum = 0;
    for update in updates {
        if !is_update_ok(update, rules) {
            let fixed_update = fix_update(update, rules);
            sum += fixed_update.middle_page();
        }
    }
    println!("Day 05, part 2: {}", sum);
}

fn is_update_ok(update: &Update, rules: &Vec<Rule>) -> bool {
    let pages_set: HashSet<i64> = HashSet::from_iter(update.pages.iter().map(|i| *i));

    for rule in rules {
        if pages_set.contains(&rule.x) && pages_set.contains(&rule.y) {
            // both pages are in the set

            let x_found = update.pages.iter().position(|n| *n == rule.x).unwrap();
            let y_found = update.pages.iter().position(|n| *n == rule.y).unwrap();
            if x_found > y_found {
                return false;
            }
        }
    }
    true
}

fn fix_update(update: &Update, rules: &Vec<Rule>) -> Update {
    let pages_set: HashSet<i64> = HashSet::from_iter(update.pages.iter().map(|i| *i));

    let relevant_rules = rules
        .iter()
        .filter(|r| pages_set.contains(&r.x) && pages_set.contains(&r.y))
        .collect_vec();

    let mut pages: Vec<i64> = Vec::from_iter(pages_set.iter().map(|i| *i));

    // sort the pages based on the rules
    pages.sort_by(|a, b| {
        let rule = relevant_rules.iter().find(|r| r.is_valid(*a, *b)).unwrap();
        if rule.x == *a {
            return std::cmp::Ordering::Less;
        }
        if rule.y == *a {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    });

    Update { pages }
}
