use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

use crate::util::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Eq)]
enum BoxMatch {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    boxes: Vec<(Position, Position)>,
}

impl Area {
    fn from(area: &[&str], twice_wide: bool) -> Area {
        let mut maxx = area[0].len() as i32;
        if twice_wide {
            maxx *= 2;
        }
        let maxy = area.len() as i32;
        let mut walls = HashSet::new();
        let mut goods = HashSet::new();
        let mut robot = Position::new(0, 0);
        let mut boxes = Vec::new();

        for (y, line) in area.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'O' => {
                        if !twice_wide {
                            goods.insert(Position::new(x as i32, y as i32));
                        } else {
                            boxes.push((
                                Position::new((x * 2) as i32, y as i32),
                                Position::new((x * 2) as i32 + 1, y as i32),
                            ));
                        }
                    }
                    '#' => {
                        if !twice_wide {
                            walls.insert(Position::new(x as i32, y as i32));
                        } else {
                            walls.insert(Position::new((x * 2) as i32, y as i32));
                            walls.insert(Position::new((x * 2) as i32 + 1, y as i32));
                        }
                    }
                    '@' => {
                        if !twice_wide {
                            robot = Position::new(x as i32, y as i32);
                        } else {
                            robot = Position::new((x * 2) as i32, y as i32);
                        }
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
            boxes,
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
                } else if let Some((p1, p2)) = self.contains_box(&pos) {
                    if p1 == pos {
                        result.push('[');
                    } else if p2 == pos {
                        result.push(']');
                    }
                } else if self.robot == pos {
                    result.push('@');
                } else {
                    result.push('.');
                }
            }
            println!("{result}");
        }
    }

    fn is_empty(&self, pos: &Position) -> bool {
        if self.walls.contains(pos) {
            false
        } else if self.goods.contains(pos) {
            false
        } else if self.boxes.iter().any(|(p1, p2)| *p1 == *pos || *p2 == *pos) {
            false
        } else {
            true
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
        } else if let Some((box_pos1, box_pos2)) = self.contains_box(&new_robot_pos) {
            if self.can_move_box_part(&box_pos1, direction)
                && self.can_move_box_part(&box_pos2, direction)
            {
                self.do_move_box(&box_pos1, &box_pos2, direction);
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

    fn contains_box(&self, pos: &Position) -> Option<(Position, Position)> {
        for (box1, box2) in &self.boxes {
            if *box1 == *pos || *box2 == *pos {
                return Some((*box1, *box2));
            }
        }
        None
    }

    fn can_move_box_part(&self, pos: &Position, direction: Direction) -> bool {
        let new_pos = match direction {
            Direction::North => Position::new(pos.x, pos.y - 1),
            Direction::South => Position::new(pos.x, pos.y + 1),
            Direction::West => Position::new(pos.x - 1, pos.y),
            Direction::East => Position::new(pos.x + 1, pos.y),
        };

        if self.walls.contains(&new_pos) {
            // Can't move
            false
        } else if let Some((box_pos1, box_pos2)) = self.contains_box(&new_pos) {
            match direction {
                Direction::West | Direction::East => {
                    // that is easy - just check if we can move the box part
                    return self.can_move_box_part(&new_pos, direction);
                }

                Direction::North | Direction::South => {
                    // can we move the complete box ? both parts ?
                    return self.can_move_box_part(&box_pos1, direction)
                        && self.can_move_box_part(&box_pos2, direction);
                }
            }
        } else {
            true
        }
    }

    fn do_move_box(&mut self, pos1: &Position, pos2: &Position, direction: Direction) -> bool {
        let (new_pos1, new_pos2) = match direction {
            Direction::North => (
                Position::new(pos1.x, pos1.y - 1),
                Position::new(pos2.x, pos2.y - 1),
            ),
            Direction::South => (
                Position::new(pos1.x, pos1.y + 1),
                Position::new(pos2.x, pos2.y + 1),
            ),
            Direction::West => (
                Position::new(pos1.x - 1, pos1.y),
                Position::new(pos2.x - 1, pos2.y),
            ),
            Direction::East => (
                Position::new(pos1.x + 1, pos1.y),
                Position::new(pos2.x + 1, pos2.y),
            ),
        };

        let mut move_box = false;
        if self.walls.contains(&new_pos1) || self.walls.contains(&new_pos2) {
            // Can't move
        } else {
            match direction {
                Direction::West => {
                    if let Some((lp1, lp2)) = self.contains_box(&new_pos1) {
                        move_box = self.do_move_box(&lp1, &lp2, direction);
                    } else {
                        move_box = true;
                    }
                }
                Direction::East => {
                    if let Some((rp1, rp2)) = self.contains_box(&new_pos2) {
                        move_box = self.do_move_box(&rp1, &rp2, direction);
                    } else {
                        move_box = true;
                    }
                }
                Direction::North | Direction::South => {
                    match (self.contains_box(&new_pos1), self.contains_box(&new_pos2)) {
                        (Some((lp1, lp2)), Some((rp1, rp2))) => {
                            if lp1 == rp1 && lp2 == rp2 {
                                // same box
                                move_box = self.do_move_box(&lp1, &lp2, direction);
                            } else {
                                move_box = self.do_move_box(&lp1, &lp2, direction)
                                    && self.do_move_box(&rp1, &rp2, direction);
                            }
                        }
                        (Some((lp1, lp2)), None) => {
                            move_box = self.do_move_box(&lp1, &lp2, direction);
                        }
                        (None, Some((rp1, rp2))) => {
                            move_box = self.do_move_box(&rp1, &rp2, direction);
                        }
                        (None, None) => {
                            move_box = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if move_box {
            self.boxes.retain(|(p1, p2)| *p1 != *pos1 && *p2 != *pos2);
            self.boxes.push((new_pos1, new_pos2));
        }
        move_box
    }

    fn calc_sum_gps_coords(&self) -> i32 {
        let mut result = 0;
        for good in &self.goods {
            let coord = good.x + good.y * 100;
            result += coord;
        }

        result
    }

    fn calc_sum_box_gps_coords(&self) -> i32 {
        let mut result = 0;
        for (p1, p2) in &self.boxes {
            // let dy = p1.y.min(self.maxy - p1.y - 1);
            // let dx1 = p1.x.min(self.maxx - p1.x - 1);
            // let dx2 = p2.x.min(self.maxx - p2.x - 1);
            // let dx = dx1.min(dx2);
            // let dy = p1.y;
            let dx = p1.x;
            let dy = p1.y;

            let coord = dx + dy * 100;
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

    part1(&area, &moves);

    part2(&area, &moves);
}

fn part1(area_str: &[&str], moves: &[Direction]) {
    let mut area = Area::from(area_str, false);

    // area.print();

    for direction in moves {
        area.move_robot(*direction);
        // area.print();
    }

    let result = area.calc_sum_gps_coords();

    println!("Day15 part 1: {:?}", result);
}

fn part2(area_str: &[&str], moves: &[Direction]) {
    let mut area = Area::from(area_str, true);

    // area.print();

    for direction in moves {
        area.move_robot(*direction);
        // area.print();
    }

    let result = area.calc_sum_box_gps_coords();

    println!("Day15 part 2: {:?}", result);
}
