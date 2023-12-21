// day21

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

use crate::util::{
    io,
    matrix::{self, Direction, Matrix, Position},
    parse,
};

const EMPTY: char = '.';
const BLOCK: char = '#';

pub fn day21() {
    println!("hello day21");

    let lines = io::read_lines("./src/day21/21.data").unwrap();

    let mut start_pos: Position = (0, 0);
    let mut area = Matrix::new(lines[0].len(), lines.len(), EMPTY);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => area.set((x, y), BLOCK),
                'S' => start_pos = (x, y), // handle start-pos as '.'   EMPTY
                _ => panic!("bad input"),
            }
        }
    }
    // println!("{:?}", start_pos);

    let result_a = solve(&area, start_pos);
    println!("Result A: {result_a}")
}

fn solve(area: &Matrix<char>, start_pos: (usize, usize)) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut next_positions: HashSet<Position> = HashSet::new();
    positions.insert(start_pos);

    for step in 0..64 {
        next_positions.clear();
        for pos in positions.iter() {
            for next_pos in area.get_neighbours(pos, |val| *val == EMPTY) {
                next_positions.insert(next_pos);
            }
        }

        positions.clear();
        for pos in next_positions.iter() {
            positions.insert(*pos);
        }
    }

    // println!("{:?}", positions);

    let count = positions.iter().count();
    count
}
