use std::collections::HashSet;

use crate::util::io;
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Hash)]
struct Position {
    x: i64,
    y: i64,
}

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

    fn guard_wallk(mut self) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.guard_pos.clone());

        let mut pos = self.guard_pos.clone();
        let mut go_one = true;
        while go_one {
            let next_pos = match self.guard_dir {
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

            if let Some(new_line) = self.area.get(next_pos.y as usize) {
                if let Some(place) = new_line.chars().nth(next_pos.x as usize) {
                    if place == '#' {
                        // turn right on block
                        self.guard_dir = match self.guard_dir {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                    } else {
                        pos = next_pos;
                        visited.insert(pos.clone());
                    }
                } else {
                    go_one = false;
                }
            } else {
                go_one = false;
            }
        }
        visited.len()
    }
}

pub fn day06() {
    let lines = io::read_lines("./src/day06/06.data").unwrap();

    let board = Board::new(lines);
    let visited = board.guard_wallk();
    println!("Day 06: Part 1) = {:?}", visited);
}
