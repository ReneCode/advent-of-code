// day07

use std::{collections::HashSet, hash::Hash};

use crate::util::io;

#[derive(Debug)]
struct Rule {
    color: String,
    contains: Vec<(String, u32)>,
}

pub fn day07() {
    let lines = io::read_lines("07.data").unwrap();

    let rules = parse_rules(&lines);

    let colors = get_all_base_colors(&rules, "shiny gold");
    println!("A: {:?}", colors.len());
}

fn get_all_base_colors(rules: &Vec<Rule>, search_color: &str) -> HashSet<String> {
    let mut base_colors: HashSet<String> = HashSet::new();
    let mut colors = get_base_colors(rules, search_color);
    while !colors.is_empty() {
        let mut new_colors = Vec::new();
        for color in colors {
            base_colors.insert(color.clone());

            let mut cs = get_base_colors(rules, &color);
            new_colors.append(&mut cs);
        }
        colors = new_colors;
    }
    base_colors
}

fn get_base_colors(rules: &Vec<Rule>, search_color: &str) -> Vec<String> {
    let mut base_colors: Vec<String> = Vec::new();
    for rule in rules {
        for (color, _count) in &rule.contains {
            if search_color == color {
                base_colors.push(rule.color.clone());
            }
        }
    }
    base_colors
}

fn parse_rules(lines: &Vec<String>) -> Vec<Rule> {
    let mut rules = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let color = parts[0].to_string();
        if parts[1] == "no other bags." {
            rules.push(Rule {
                color,
                contains: Vec::new(),
            });
            continue;
        }
        let contains = parts[1]
            .split(", ")
            .map(|s| {
                let parts: Vec<&str> = s.split(" ").collect();
                let count = parts[0].parse::<u32>().unwrap();
                let color = parts[1..3].join(" ");
                (color, count)
            })
            .collect();

        rules.push(Rule { color, contains });
    }

    rules
}
