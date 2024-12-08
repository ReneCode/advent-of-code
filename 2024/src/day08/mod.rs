use std::collections::HashSet;

use crate::util::io;
use itertools::Itertools;

type Number = i64;

struct BoundingBox {
    min: Position,
    max: Position,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: Number,
    y: Number,
}

impl Position {
    fn new(x: Number, y: Number) -> Position {
        Position { x, y }
    }
    fn add(&self, other: &Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }
    fn sub(&self, other: &Position) -> Position {
        Position::new(self.x - other.x, self.y - other.y)
    }
    fn factor(&self, factor: Number) -> Position {
        Position::new(self.x * factor, self.y * factor)
    }
    fn inside(&self, bbox: &BoundingBox) -> bool {
        self.x >= bbox.min.x && self.x <= bbox.max.x && self.y >= bbox.min.y && self.y <= bbox.max.y
    }
}

#[derive(Debug)]
struct Antenna {
    position: Position,
    frequence: char,
}

const EMPTY: char = '.';

pub fn day08() {
    let lines = io::read_lines("./src/day08/08.data").unwrap();
    let line_count = lines.len();
    let line_len = lines[0].len();

    let mut antennas = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != EMPTY && c != '#' {
                antennas.push(Antenna {
                    position: Position::new(x as Number, y as Number),
                    frequence: c,
                });
            }
        }
    }

    part1(&antennas, line_len, line_count);

    part2(&antennas, line_len, line_count);
}

fn part1(antennas: &[Antenna], line_len: usize, line_count: usize) {
    let mut anti_nodes: HashSet<Position> = HashSet::new();
    let groups = antennas
        .iter()
        .sorted_by(|a, b| a.frequence.cmp(&b.frequence))
        .chunk_by(|a| a.frequence);
    for (_frequence, ant_of_one_group) in groups.into_iter() {
        let positions = ant_of_one_group.map(|a| &a.position).collect_vec();
        let antenna_pairs = positions.iter().combinations(2).collect_vec();
        for pair in antenna_pairs {
            let a = pair[0];
            let b = pair[1];
            let delta = a.sub(b);
            let check_nodes = vec![a.add(&delta), b.sub(&delta)];
            // add, if inside the grid
            for node in check_nodes {
                if node.x >= 0
                    && node.x < line_len as Number
                    && node.y >= 0
                    && node.y < line_count as Number
                {
                    anti_nodes.insert(node);
                }
            }
        }
    }
    println!("Day07 part 1: {:?}", anti_nodes.len());
}

fn part2(antennas: &[Antenna], line_len: usize, line_count: usize) {
    let bbox = BoundingBox {
        min: Position::new(0, 0),
        max: Position::new(line_len as Number - 1, line_count as Number - 1),
    };
    let mut anti_nodes: HashSet<Position> = HashSet::new();
    let groups = antennas
        .iter()
        .sorted_by(|a, b| a.frequence.cmp(&b.frequence))
        .chunk_by(|a| a.frequence);
    for (_frequence, ant_of_one_group) in groups.into_iter() {
        let positions = ant_of_one_group.map(|a| &a.position).collect_vec();
        let antenna_pairs = positions.iter().combinations(2).collect_vec();
        for pair in antenna_pairs {
            let a = pair[0];
            let b = pair[1];
            let delta = a.sub(b);

            // let mut check_nodes: Vec<   Position> = Vec::new();
            let mut i = 0;
            loop {
                let node = a.add(&delta.factor(i));
                if !node.inside(&bbox) {
                    break;
                }
                anti_nodes.insert(node);
                i += 1;
            }
            i = 0;
            loop {
                let node = a.sub(&delta.factor(i));
                if !node.inside(&bbox) {
                    break;
                }
                anti_nodes.insert(node);
                i += 1;
            }

            // // add, if inside the grid
            // for node in check_nodes {
            //     if node.x >= 0
            //         && node.x < line_len as Number
            //         && node.y >= 0
            //         && node.y < line_count as Number
            //     {
            //         anti_nodes.insert(node);
            //     }
            // }
        }
    }

    println!("Day07 part 2: {:?}", anti_nodes.len());
}
