// day17

use std::{
    collections::{btree_set::Difference, HashMap, HashSet},
    path::Path,
};

use itertools::Itertools;

use crate::util::{
    io,
    matrix::{calc_direction, Direction, Matrix, Position},
    parse,
};

type PositionStepItem = (Position, i32);

struct PathFinder<'a> {
    area: &'a Matrix<i32>,
    // waiting_items: Vec<PositionStepItem>,
}

impl<'a> PathFinder<'a> {
    fn new(area: &'a Matrix<i32>) -> Self {
        PathFinder {
            area: area,
            // waiting_items: Vec::new(),
        }
    }

    fn get_forbidden_direction(
        &self,
        parents: &HashMap<Position, Position>,
        last_pos: &Position,
    ) -> Option<Direction> {
        // check, if the last three directions are the same

        let mut pos = last_pos;
        let last_direction: Option<Direction> = None;
        for _ in 0..3 {
            if let Some(prev_pos) = parents.get(pos) {
                if let Some(direction) = calc_direction(prev_pos, pos) {
                    if let Some(last_dir) = last_direction {
                        if last_dir != direction {
                            return None;
                        }
                    }
                } else {
                    return None;
                }
            } else {
                panic!("bad parent positions")
            }
        }
        last_direction
    }

    fn find_minimal_path(&self, start: (usize, usize)) {
        let mut distances: HashMap<Position, i32> = HashMap::new();
        for x in 0..self.area.xlen() {
            for y in 0..self.area.ylen() {
                distances.insert((x, y), i32::MAX);
            }
        }
        distances.insert(start, 0);
        let mut queue: Vec<Position> = Vec::new();
        queue.push(start);

        let mut parents: HashMap<Position, Position> = HashMap::new();
        parents.insert(start, start);

        let stop_pos = (self.area.xlen() - 1, self.area.ylen() - 1);

        while (queue.len() > 0) {
            // get minimal waiting
            let min_steps = queue
                .iter()
                .map(|pos| distances.get(pos))
                .filter(|op| op.is_some())
                .map(|op| op.unwrap())
                .min()
                .unwrap();
            let index = queue
                .iter()
                .position(|pos| {
                    if let Some(steps) = distances.get(pos) {
                        return steps == min_steps;
                    }
                    return false;
                })
                .unwrap();
            let current_pos = queue[index];
            queue.remove(index);

            let current_distance = *distances.get(&current_pos).unwrap();

            let next_positions = self.get_neighbours(current_pos);
            if let Some(forbidden_direction) = self.get_forbidden_direction(&parents, &current_pos)
            {
                println!("do not walk: {:?}", forbidden_direction);
            }

            for (next_pos, next_direction) in next_positions {
                // if !parents.contains_key(&next_pos) {
                let tile = self.area.get(next_pos);
                let distance = current_distance + tile;
                let next_distance = *distances.get(&next_pos).unwrap();
                if queue.contains(&next_pos) && distance < next_distance {
                    distances.insert(next_pos, distance);
                    parents.insert(next_pos, current_pos);
                } else if !parents.contains_key(&next_pos) {
                    distances.insert(next_pos, distance);
                    parents.insert(next_pos, current_pos);
                    queue.push(next_pos);
                }
                // }
            }
        }

        let min = distances.get(&stop_pos).unwrap();

        let mut min_path: Vec<Position> = Vec::new();
        let mut pos = stop_pos;
        while pos != start {
            min_path.push(pos);
            if let Some(parent_pos) = parents.get(&pos) {
                pos = *parent_pos;
            } else {
                panic!("broken path")
            }
        }

        min_path.reverse();
        println!("solve {min}");
        println!("solve {:?}", min_path);
    }

    fn get_neighbours(&self, pos: Position) -> Vec<(Position, Direction)> {
        let mut result: Vec<(Position, Direction)> = Vec::new();
        if let Some(next) = self.area.next_pos(pos, &Direction::UP) {
            result.push((next, Direction::UP));
        }
        if let Some(next) = self.area.next_pos(pos, &Direction::RIGHT) {
            result.push((next, Direction::RIGHT));
        }
        if let Some(next) = self.area.next_pos(pos, &Direction::DOWN) {
            result.push((next, Direction::DOWN));
        }
        if let Some(next) = self.area.next_pos(pos, &Direction::LEFT) {
            result.push((next, Direction::LEFT));
        }
        result
    }
}

pub fn day17() {
    println!("hello day17");

    let lines = io::read_lines("./src/day17/17-example.data").unwrap();
    let xlen = lines[0].len();
    let ylen = lines.len();
    let mut area: Matrix<i32> = Matrix::new(xlen, ylen, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let nr = c as i32 - '0' as i32;
            area.set((x, y), &nr);
        }
    }

    // println!("area {:?}", area);

    let result_a: i32 = part_a(&area);
}

fn part_a(area: &Matrix<i32>) -> i32 {
    let path_finder = PathFinder::new(area);
    path_finder.find_minimal_path((0, 0));

    0
}
