// day08

use std::collections::HashMap;

use crate::util::{io, parse};

enum State {
    ReadInstructions,
    ReadNodes,
}

#[derive(Debug)]
struct Node {
    // name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(_name: &str, left: &str, right: &str) -> Self {
        Node {
            // name: name.to_string(),
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

    let result_a = travel(instructions, &nodes, "AAA", "ZZZ");

    println!("Result A: {result_a}");
}

fn travel(instructions: String, nodes: &HashMap<String, Node>, start: &str, stop: &str) -> usize {
    // println!("{start} / {instruction_idx}");
    let mut idx: usize = 0;
    let mut node_name = start;
    while node_name != stop {
        let node = nodes.get(node_name).unwrap();
        let instruction = instructions.chars().nth(idx % instructions.len()).unwrap();
        match instruction {
            'L' => {
                idx += 1;
                node_name = &node.left;
            }
            'R' => {
                idx += 1;
                node_name = &node.right;
            }
            _ => panic!("bad instruction"),
        }
    }
    idx
}
