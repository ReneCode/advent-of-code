// day08

use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{io, parse};

enum State {
    ReadInstructions,
    ReadNodes,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: &str, left: &str, right: &str) -> Self {
        Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }

    fn get_next(&self, next: char) -> &str {
        match next {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("bad instruction"),
        }
    }
}

pub fn day08() {
    println!("hello day08");

    let lines = io::read_lines("./src/day08/08.data").unwrap();

    let mut state = State::ReadInstructions;
    let mut instructions = String::new();
    let mut nodes: Vec<Node> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        match state {
            State::ReadInstructions => {
                instructions = line.clone();
                state = State::ReadNodes;
            }
            State::ReadNodes => {
                let tok = parse::to_str(&line, '=');
                let name = tok[0];
                let targets = tok[1].replace('(', "").replace(')', "");
                let tok = parse::to_str(&targets, ',');
                let node = Node::new(name, tok[0], tok[1]);
                nodes.push(node);
            }
        }
    }

    // println!("read: {:?} / {:?}", instructions, nodes);

    // let result_a = part_a(instructions.as_str(), &nodes, "AAA", "ZZZ");
    // println!("Result A: {result_a}");

    let result_b = part_b(instructions.as_str(), &nodes, "A", "Z");
    println!("Result B: {result_b}");
}

fn part_a(instructions: &str, nodes: &Vec<Node>, start: &str, stop: &str) -> usize {
    // println!("{start} / {instruction_idx}");
    let mut idx: usize = 0;
    let mut node_name = start;
    while node_name != stop {
        let node = nodes.iter().find(|n| n.name == node_name).unwrap();
        let instruction = instructions.chars().nth(idx % instructions.len()).unwrap();
        node_name = node.get_next(instruction);
        idx += 1;
    }
    idx
}

fn part_b(instructions: &str, all_nodes: &Vec<Node>, start: &str, stop: &str) -> usize {
    // println!("{start} / {instruction_idx}");
    let mut idx: usize = 0;
    let mut nodes = all_nodes
        .iter()
        .filter(|n| n.name.ends_with(start))
        .collect_vec();
    while !nodes.iter().all(|n| n.name.ends_with(&stop)) {
        let instruction = instructions.chars().nth(idx % instructions.len()).unwrap();
        idx += 1;
        nodes = nodes
            .iter()
            .map(|n| {
                let next_name = n.get_next(instruction);
                let next_node = all_nodes.iter().find(|n| n.name == next_name).unwrap();
                next_node
            })
            .collect_vec();
        // println!(">> {:?}", nodes.iter().map(|n| &n.name))
    }
    idx
}
