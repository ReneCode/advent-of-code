use std::collections::HashSet;

use crate::util::io;
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct TurningPoint {
    pos: Position,
    dir: Direction,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Board {
    area: Vec<String>,
    guard_pos: Position,
    guard_dir: Direction,
}

const BLOCK: char = '#';
const GUARD: char = '^';

enum WalkResult {
    LeaveArea(usize),
    Loop,
}

impl Board {
    fn new(lines: Vec<String>) -> Self {
        let mut guard_pos = Position { x: 0, y: 0 };
        for (y, line) in lines.iter().enumerate() {
            if let Some(x) = line.chars().position(|c| c == GUARD) {
                guard_pos = Position {
                    x: x as i64,
                    y: y as i64,
                };
                break;
            }
        }
        Board {
            area: lines,
            guard_pos,
            guard_dir: Direction::Up,
        }
    }

    fn guard_wallk(&self) -> WalkResult {
        let mut visited = HashSet::new();
        visited.insert(self.guard_pos.clone());
        let mut turning_positions = HashSet::new();

        let area_len = self.area.get(0).unwrap().len() as i64;
        let area_height = self.area.len() as i64;

        let mut pos = self.guard_pos.clone();
        let mut dir = self.guard_dir.clone();
        loop {
            let next_pos = match dir {
                Direction::Up => Position {
                    x: pos.x,
                    y: pos.y - 1,
                },
                Direction::Down => Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
                Direction::Left => Position {
                    x: pos.x - 1,
                    y: pos.y,
                },
                Direction::Right => Position {
                    x: pos.x + 1,
                    y: pos.y,
                },
            };

            if next_pos.x < 0
                || next_pos.y < 0
                || next_pos.x >= area_len
                || next_pos.y >= area_height
            {
                return WalkResult::LeaveArea(visited.len());
            }

            let place = self
                .area
                .get(next_pos.y as usize)
                .unwrap()
                .chars()
                .nth(next_pos.x as usize)
                .unwrap();
            if place == '#' {
                // turn right on block
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                let turning_point = TurningPoint {
                    pos: pos.clone(),
                    dir: dir.clone(),
                };
                if turning_positions.contains(&turning_point) {
                    return WalkResult::Loop;
                } else {
                    turning_positions.insert(turning_point);
                }
            } else {
                pos = next_pos;
                visited.insert(pos.clone());
            }
        }
        WalkResult::Loop
    }
}

pub fn day06() {
    let lines = io::read_lines("./src/day06/06.data").unwrap();

    let board = Board::new(lines);
    if let WalkResult::LeaveArea(visited) = board.guard_wallk() {
        println!("Day 06: Part 1) = {:?}", visited);
    }

    let lines = io::read_lines("./src/day06/06.data").unwrap();

    let mut replace = "".to_string();
    replace.push(BLOCK);
    let mut replace_results_loop = 0;
    // brute force
    for y in 0..lines.len() {
        for x in 0..lines.get(0).unwrap().len() {
            let current = lines[y].chars().nth(x).unwrap();
            if current != GUARD && current != BLOCK {
                if lines[y].chars().nth(x).unwrap() != GUARD {
                    let mut modifed_lines = lines.clone();
                    let mut mut_line = modifed_lines.get_mut(y).unwrap();
                    mut_line.replace_range(x..x + 1, &replace);

                    let board = Board::new(modifed_lines);
                    if let WalkResult::Loop = board.guard_wallk() {
                        println!("loop when {} / {}", x, y);
                        replace_results_loop += 1;
                    }
                }
            }
        }
    }
    println!("Day 06: Part 2) = {:?}", replace_results_loop);
}
