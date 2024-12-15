use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

use crate::util::io;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct Area {
    maxx: i32,
    maxy: i32,
    walls: HashSet<Position>,
    goods: HashSet<Position>,
    robot: Position,
}

impl Area {
    fn from(area: &[&str]) -> Area {
        let maxx = area[0].len() as i32;
        let maxy = area.len() as i32;
        let mut walls = HashSet::new();
        let mut goods = HashSet::new();
        let mut robot = Position::new(0, 0);

        for (y, line) in area.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'O' => {
                        goods.insert(Position::new(x as i32, y as i32));
                    }
                    '#' => {
                        walls.insert(Position::new(x as i32, y as i32));
                    }
                    '@' => {
                        robot = Position::new(x as i32, y as i32);
                    }
                    _ => {}
                }
            }
        }

        Area {
            maxx,
            maxy,
            walls,
            goods,
            robot,
        }
    }

    fn print(&self) {
        for y in 0..self.maxy {
            let mut result = String::new();
            for x in 0..self.maxx {
                let pos = Position::new(x, y);
                if self.walls.contains(&pos) {
                    result.push('#');
                } else if self.goods.contains(&pos) {
                    result.push('O');
                } else if self.robot == pos {
                    result.push('@');
                } else {
                    result.push('.');
                }
            }
            println!("{result}");
        }
    }

    fn move_robot(&mut self, direction: Direction) -> bool {
        let new_robot_pos = match direction {
            Direction::North => Position::new(self.robot.x, self.robot.y - 1),
            Direction::South => Position::new(self.robot.x, self.robot.y + 1),
            Direction::West => Position::new(self.robot.x - 1, self.robot.y),
            Direction::East => Position::new(self.robot.x + 1, self.robot.y),
        };
        if self.walls.contains(&new_robot_pos) {
            // Can't move
            false
        } else if self.goods.contains(&new_robot_pos) {
            if self.move_good(&new_robot_pos, direction) {
                self.robot = new_robot_pos;
                true
            } else {
                false
            }
        } else {
            // free space - just move robot
            self.robot = new_robot_pos;
            true
        }
    }

    fn move_good(&mut self, pos: &Position, direction: Direction) -> bool {
        let new_pos = match direction {
            Direction::North => Position::new(pos.x, pos.y - 1),
            Direction::South => Position::new(pos.x, pos.y + 1),
            Direction::West => Position::new(pos.x - 1, pos.y),
            Direction::East => Position::new(pos.x + 1, pos.y),
        };

        if self.walls.contains(&new_pos) {
            // Can't move
            false
        } else if self.goods.contains(&new_pos) {
            if self.move_good(&new_pos, direction) {
                let ok = self.goods.remove(pos);
                assert!(ok); // We should always find the good
                self.goods.insert(new_pos);
                true
            } else {
                false
            }
        } else {
            // free space - just move robot
            let ok = self.goods.remove(pos);
            assert!(ok); // We should always find the good
            self.goods.insert(new_pos);
            true
        }
    }

    fn calc_sum_gps_coords(&self) -> i32 {
        let mut result = 0;
        for good in &self.goods {
            let coord = good.x + good.y * 100;
            result += coord;
        }
        result
    }
}

pub fn day15() {
    let lines = io::read_lines("./src/day15/15.data").unwrap();
    let all_lines = lines.join("\n");
    let groups = all_lines.split("\n\n").collect_vec();

    let area = groups[0].split("\n").collect_vec();
    let mut area = Area::from(&area);

    let moves = groups[1].split("\n").collect_vec();
    let moves = moves
        .join("")
        .chars()
        .map(|c| match c {
            'v' => Direction::South,
            '^' => Direction::North,
            '<' => Direction::West,
            '>' => Direction::East,
            _ => {
                panic!("Invalid direction: {}", c);
            }
        })
        .collect_vec();

    part1(&mut area, &moves);
}

fn part1(area: &mut Area, moves: &[Direction]) {
    area.print();

    for direction in moves {
        area.move_robot(*direction);
        // area.print();
    }

    let result = area.calc_sum_gps_coords();

    println!("Day15 part 1: {:?}", result);
}
