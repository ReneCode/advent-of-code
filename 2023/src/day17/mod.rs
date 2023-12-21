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
        let mut last_direction: Option<Direction> = None;
        for _ in 0..3 {
            if *pos == (0, 0) {
                return None;
            }
            if let Some(prev_pos) = parents.get(pos) {
                if let Some(direction) = calc_direction(prev_pos, pos) {
                    if let Some(last_dir) = last_direction {
                        if last_dir != direction {
                            return None;
                        }
                    } else {
                        last_direction = Some(direction);
                    }
                } else {
                    return None;
                }
                pos = prev_pos;
            } else {
                panic!("bad parent positions")
            }
        }
        last_direction
    }

    fn get_minimal_unvisited_cost(
        &self,
        costs: &HashMap<Position, i32>,
        visited: &Vec<Position>,
    ) -> (Position, i32) {
        let mut min_cost = i32::MAX;
        let mut min_pos = (0, 0);
        for pos in costs.keys().filter(|p| !visited.contains(p)) {
            if let Some(cost) = costs.get(pos) {
                if *cost < min_cost {
                    min_cost = *cost;
                    min_pos = *pos;
                }
            } else {
                panic!("broken cost map")
            }
        }

        (min_pos, min_cost)
    }

    fn find_minimal_path(&self, start: (usize, usize)) {
        let mut costs: HashMap<Position, i32> = HashMap::new();
        for x in 0..self.area.xlen() {
            for y in 0..self.area.ylen() {
                costs.insert((x, y), i32::MAX);
            }
        }
        costs.insert(start, 0);

        let mut parents: HashMap<Position, Position> = HashMap::new();
        parents.insert(start, start);

        let stop_pos = (self.area.xlen() - 1, self.area.ylen() - 1);

        let total_positions = self.area.xlen() * self.area.ylen();
        let mut visited: Vec<Position> = Vec::new();

        while (visited.len() < total_positions) {
            // get minimal waiting
            let (current_pos, current_cost) = self.get_minimal_unvisited_cost(&costs, &visited);

            let mut next_positions = self.get_neighbours(current_pos);
            if let Some(forbidden_direction) = self.get_forbidden_direction(&parents, &current_pos)
            {
                next_positions = next_positions
                    .iter()
                    .filter(|p| p.1 != forbidden_direction)
                    .map(|p| *p)
                    .collect_vec();
                println!(
                    "do not walk: {:?} from pos:{:?}",
                    forbidden_direction, current_pos,
                );
            }

            for (next_pos, next_direction) in next_positions {
                let cost = self.area.get(next_pos);
                let distance = current_cost + cost;
                let next_distance = *costs.get(&next_pos).unwrap();
                if distance < next_distance {
                    costs.insert(next_pos, distance);
                    parents.insert(next_pos, current_pos);
                }
            }
            visited.push(current_pos);
        }

        for y in 0..self.area.ylen() {
            for x in 0..self.area.xlen() {
                let pos = (x, y);
                let d = costs.get(&pos).unwrap();
                // println!("{:?} {}", pos, d);
            }
        }

        let min = costs.get(&stop_pos).unwrap();

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
        if let Some(next) = self.area.next_pos(pos, Direction::UP) {
            result.push((next, Direction::UP));
        }
        if let Some(next) = self.area.next_pos(pos, Direction::RIGHT) {
            result.push((next, Direction::RIGHT));
        }
        if let Some(next) = self.area.next_pos(pos, Direction::DOWN) {
            result.push((next, Direction::DOWN));
        }
        if let Some(next) = self.area.next_pos(pos, Direction::LEFT) {
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
            area.set((x, y), nr);
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
