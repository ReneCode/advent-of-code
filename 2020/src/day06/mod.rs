// day06

use std::collections::HashSet;

use itertools::Itertools;

use crate::util::io;

pub fn day06() {
    let lines = io::read_lines("06.data").unwrap();
    let one_string = lines.iter().join("\n");
    let groups: Vec<&str> = one_string.split("\n\n").collect();
    let questions: Vec<usize> = groups
        .iter()
        .map(|group| get_questions_from_group(group))
        .collect();
    let sum_questions: usize = questions.iter().sum();
    println!("A: sum of questions: {}", sum_questions);
}

fn get_questions_from_group(group: &str) -> usize {
    let persons: Vec<&str> = group.split("\n").collect();
    let mut questions: HashSet<char> = HashSet::new();
    for person in persons {
        for c in person.chars() {
            questions.insert(c);
        }
    }
    questions.len()
}
