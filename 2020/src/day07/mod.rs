// day07

use std::collections::HashSet;

use crate::util::io;

#[derive(Debug)]
struct Rule {
    color: String,
    contains: Vec<(String, u32)>,
}

pub fn day07() {
    let lines = io::read_lines("07.data").unwrap();

    let rules = parse_rules(&lines);

    let colors = collect_base_colors(&rules, "shiny gold");
    println!("A: {:?}", colors.len());

    let count_bags = count_bags(&rules, "shiny gold");
    println!("B: {:?}", count_bags);
}

fn count_bags(rules: &Vec<Rule>, color: &str) -> u32 {
    let rule = rules.iter().find(|r| r.color == color).unwrap();
    recursive_count_bags(rules, 1, rule)
}

fn recursive_count_bags(rules: &Vec<Rule>, count: u32, rule: &Rule) -> u32 {
    let mut all_count = 0;
    for (child_color, child_count) in &rule.contains {
        all_count += count * child_count;
        if let Some(sub_rule) = rules.iter().find(|r| r.color == *child_color) {
            let c = recursive_count_bags(rules, count * child_count, sub_rule);
            all_count += c;
        }
    }

    all_count
}

fn collect_base_colors(rules: &Vec<Rule>, color: &str) -> HashSet<String> {
    let mut all_base_colors: HashSet<String> = HashSet::new();

    let base_colors = get_base_colors(rules, color);

    for color in base_colors {
        all_base_colors.insert(color.clone());
        let local_base_colors = collect_base_colors(rules, &color);
        for c in local_base_colors.iter() {
            all_base_colors.insert(c.clone());
        }
    }

    all_base_colors
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
