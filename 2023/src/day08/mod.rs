// day08

use std::collections::{HashMap, HashSet};

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
}

pub fn day08() {
    println!("hello day08");

    let lines = io::read_lines("./src/day08/08.data").unwrap();

    let mut state = State::ReadInstructions;
    let mut instructions = String::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();
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
                let name = tok[0].to_string();
                let targets = tok[1].replace('(', "").replace(')', "");
                let tok = parse::to_str(&targets, ',');
                let node = Node::new(name.as_str(), tok[0], tok[1]);
                nodes.insert(name, node);
            }
        }
    }
    // println!("read: {:?} / {:?}", instructions, nodes);

    let result_a = travel(instructions, &nodes, 0, "AAA", "ZZZ");

    println!("Result A:{result_a}");
}

fn travel(
    instructions: String,
    nodes: &HashMap<String, Node>,
    instruction_idx: usize,
    start: &str,
    stop: &str,
) -> usize {
    println!("{start} / {instruction_idx}");
    if start == stop {
        return instruction_idx;
    }
    let node = nodes.get(start).unwrap();
    let instruction = instructions
        .chars()
        .nth(instruction_idx % instructions.len())
        .unwrap();
    match instruction {
        'L' => {
            return travel(instructions, nodes, instruction_idx + 1, &node.left, stop);
        }
        'R' => {
            return travel(instructions, nodes, instruction_idx + 1, &node.right, stop);
        }
        _ => panic!("bad instruction"),
    }
}
