// day06

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::io;

pub fn day06() {
    let lines = io::read_lines("06.data").unwrap();
    let one_string = lines.iter().join("\n");
    let groups: Vec<&str> = one_string.split("\n\n").collect();
    let any_questions: Vec<usize> = groups
        .iter()
        .map(|group| get_any_person_answered(group))
        .collect();
    let sum_any_questions: usize = any_questions.iter().sum();
    println!("A: sum of any questions: {}", sum_any_questions);

    let all_questions: Vec<usize> = groups
        .iter()
        .map(|group| get_all_person_answered(group))
        .collect();
    let sum_all_questions: usize = all_questions.iter().sum();

    println!("B: sum of all questions: {}", sum_all_questions);
}

fn get_any_person_answered(group: &str) -> usize {
    let persons: Vec<&str> = group.split("\n").collect();
    let mut questions: HashSet<char> = HashSet::new();
    for person in persons {
        for c in person.chars() {
            questions.insert(c);
        }
    }
    questions.len()
}

fn get_all_person_answered(group: &str) -> usize {
    let persons: Vec<&str> = group.split("\n").collect();
    let all_questions: HashSet<char> = persons.iter().map(|p| p.chars()).flatten().collect();
    let mut count: usize = 0;

    for question in all_questions {
        let mut answered = true;
        for person in persons.iter() {
            if !person.contains(question) {
                answered = false;
                break;
            }
        }
        if answered {
            // answered by all persons
            count += 1;
        }
    }
    count
}
