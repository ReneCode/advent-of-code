use std::ops::Index;

extern crate util;

const CENTER_OF_MASS: &str = "COM";

struct Orbit {
    center: String,
    outside: String,
}
impl Orbit {
    fn new(center: &str, outside: &str) -> Orbit {
        Orbit {
            center: String::from(center),
            outside: String::from(outside),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    distance: i32,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str, distance: i32) -> Self {
        Node {
            name: String::from(name),
            distance: distance,
            children: Vec::new(),
        }
    }
}

fn create_node(all_orbits: &Vec<Orbit>, name: &str, distance: i32) -> Node {
    let orbits: Vec<&Orbit> = all_orbits.iter().filter(|o| o.center == name).collect();
    let mut node = Node::new(name, distance);
    if orbits.len() > 0 {
        let mut children: Vec<Node> = Vec::new();
        for orbit in orbits {
            let child = create_node(all_orbits, orbit.outside.as_str(), distance + 1);
            children.push(child);
        }
        node.children = children;
    }
    node
}

fn sum_distance(node: &Node, total: &mut i32) {
    *total += node.distance;
    for child in node.children.iter() {
        sum_distance(&child, total)
    }
}

fn main() {
    println!("Hello, day06!");
    if let Some(orbits) = get_data("./06.data") {
        let root = create_node(&orbits, CENTER_OF_MASS, 0);
        part_1(&root);
        part_2(&root);
    }
}

fn get_data(filename: &str) -> Option<Vec<Orbit>> {
    if let Some(input) = util::io::get_lines(filename) {
        let orbits: Vec<Orbit> = input
            .iter()
            .map(|line| {
                let tok: Vec<&str> = line.split(")").collect();
                let orbit = Orbit::new(tok[0], tok[1]);
                orbit
            })
            .collect();
        Some(orbits)
    } else {
        None
    }
}

fn part_1(root: &Node) {
    let mut total: i32 = 0;
    sum_distance(&root, &mut total);
    println!("part-1 total {}", total);
}

fn part_2(root: &Node) {
    let mut way_you: Vec<&str> = Vec::new();
    collect_way(root, "YOU", &mut way_you);
    let mut way_san: Vec<&str> = Vec::new();
    collect_way(root, "SAN", &mut way_san);

    let mut shared_name = "";
    for name in way_you.iter() {
        if way_san.contains(&name) {
            shared_name = *name;
            break;
        }
    }
    if let Some(pos_you) = way_you.iter().position(|n| *n == shared_name) {
        if let Some(pos_san) = way_san.iter().position(|n| *n == shared_name) {
            println!("part-2 orbtit transfers {}", (pos_san - 1) + (pos_you - 1));
        }
    }
}

fn collect_way<'a>(root: &'a Node, target: &str, way: &mut Vec<&'a str>) -> bool {
    if root.name == target {
        true
    } else {
        for child in root.children.iter() {
            if collect_way(child, target, way) {
                way.push(child.name.as_str());
                return true;
            }
        }
        false
    }
}
