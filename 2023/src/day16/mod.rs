// day16

use std::collections::{btree_set::Difference, HashMap, HashSet};

use itertools::Itertools;

use crate::util::{
    io,
    matrix::{Direction, Matrix},
    parse,
};

pub fn day16() {
    println!("hello day16");

    let lines = io::read_lines("./src/day16/16.data").unwrap();
    let xlen = lines[0].len();
    let ylen = lines.len();
    let mut area: Matrix<char> = Matrix::new(xlen, ylen, ',');
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            area.set((x, y), &c);
        }
    }

    let result_a = solve(&area, (0, 0), Direction::RIGHT);
    println!("Result A {result_a}");

    let mut result_b = 0;
    for x in 0..area.xlen() {
        let result = solve(&area, (x, 0), Direction::DOWN);
        result_b = result_b.max(result);

        let result = solve(&area, (x, area.ylen() - 1), Direction::UP);
        result_b = result_b.max(result);
    }
    for y in 0..area.ylen() {
        let result = solve(&area, (0, y), Direction::RIGHT);
        result_b = result_b.max(result);

        let result = solve(&area, (area.xlen() - 1, y), Direction::LEFT);
        result_b = result_b.max(result);
    }

    println!("Result B {result_b}");
}

fn solve(area: &Matrix<char>, start_pos: (usize, usize), start_direction: Direction) -> usize {
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();

    let mut continue_here: Vec<((usize, usize), Direction)> = Vec::new();
    continue_here.push((start_pos, start_direction));

    while continue_here.len() > 0 {
        let mut pos = continue_here[0].0;
        let mut direction = continue_here[0].1;
        continue_here.remove(0);

        if visited.contains(&(pos, direction)) {
            continue;
        }

        let mut go_on = true;
        while go_on {
            let tile = area.get(pos);
            visited.insert((pos, direction));
            match tile {
                '.' => {
                    if let Some(next_pos) = area.next_pos(pos, &direction) {
                        pos = next_pos;
                    } else {
                        go_on = false;
                    }
                }
                '-' => match direction {
                    Direction::UP | Direction::DOWN => {
                        if let Some(next_pos) = area.next_pos(pos, &Direction::RIGHT) {
                            continue_here.push((next_pos, Direction::RIGHT));
                        }
                        if let Some(next_pos) = area.next_pos(pos, &Direction::LEFT) {
                            pos = next_pos;
                            direction = Direction::LEFT;
                        } else {
                            go_on = false
                        }
                    }
                    Direction::LEFT | Direction::RIGHT => {
                        if let Some(next_pos) = area.next_pos(pos, &direction) {
                            pos = next_pos;
                        } else {
                            go_on = false;
                        }
                    }
                },
                '|' => match direction {
                    Direction::LEFT | Direction::RIGHT => {
                        if let Some(next_pos) = area.next_pos(pos, &Direction::DOWN) {
                            continue_here.push((next_pos, Direction::DOWN));
                        }
                        if let Some(next_pos) = area.next_pos(pos, &Direction::UP) {
                            pos = next_pos;
                            direction = Direction::UP;
                        } else {
                            go_on = false;
                        }
                    }
                    Direction::UP | Direction::DOWN => {
                        if let Some(next_pos) = area.next_pos(pos, &direction) {
                            pos = next_pos;
                        } else {
                            go_on = false;
                        }
                    }
                },
                '/' => {
                    let continue_direction: HashMap<Direction, Direction> = HashMap::from([
                        (Direction::LEFT, Direction::DOWN),
                        (Direction::RIGHT, Direction::UP),
                        (Direction::DOWN, Direction::LEFT),
                        (Direction::UP, Direction::RIGHT),
                    ]);
                    let next_direction = continue_direction.get(&direction).unwrap();
                    if let Some(next_pos) = area.next_pos(pos, next_direction) {
                        pos = next_pos;
                        direction = *next_direction;
                    } else {
                        go_on = false;
                    }
                }
                '\\' => {
                    let continue_direction: HashMap<Direction, Direction> = HashMap::from([
                        (Direction::LEFT, Direction::UP),
                        (Direction::RIGHT, Direction::DOWN),
                        (Direction::DOWN, Direction::RIGHT),
                        (Direction::UP, Direction::LEFT),
                    ]);
                    let next_direction = continue_direction.get(&direction).unwrap();
                    if let Some(next_pos) = area.next_pos(pos, next_direction) {
                        pos = next_pos;
                        direction = *next_direction;
                    } else {
                        go_on = false;
                    }
                }
                _ => panic!("bad tile"),
            }
        }
    }

    let all_positions: HashSet<(usize, usize)> = visited.iter().map(|v| v.0).collect();
    let count = all_positions.len();
    count
}
