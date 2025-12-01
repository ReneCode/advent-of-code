use core::num;
use std::{collections::HashMap, hash::Hash, result, vec};

use crate::util::io;

const KEY_A: char = 'A';
const KEY_0: char = '0';
const KEY_1: char = '1';
const KEY_2: char = '2';
const KEY_3: char = '3';
const KEY_4: char = '4';
const KEY_5: char = '5';
const KEY_6: char = '6';
const KEY_7: char = '7';
const KEY_8: char = '8';
const KEY_9: char = '9';
const KEY_UP: char = '^';
const KEY_DOWN: char = 'v';
const KEY_LEFT: char = '<';
const KEY_RIGHT: char = '>';

struct MoveSolver {
    current_key: char,
    move_map: HashMap<(char, char), Vec<char>>, // (from, to) -> [moves]
}

impl MoveSolver {
    fn new() -> Self {
        let mut move_map = HashMap::new();

        move_map.insert((KEY_A, KEY_LEFT), vec![KEY_DOWN, KEY_LEFT, KEY_LEFT]);
        move_map.insert((KEY_A, KEY_UP), vec![KEY_LEFT]);
        move_map.insert((KEY_A, KEY_RIGHT), vec![KEY_DOWN]);
        move_map.insert((KEY_A, KEY_DOWN), vec![KEY_LEFT, KEY_DOWN]);
        move_map.insert((KEY_A, KEY_A), vec![]);

        move_map.insert((KEY_UP, KEY_UP), vec![]);
        move_map.insert((KEY_UP, KEY_RIGHT), vec![KEY_DOWN, KEY_RIGHT]);
        move_map.insert((KEY_UP, KEY_A), vec![KEY_RIGHT]);
        move_map.insert((KEY_UP, KEY_LEFT), vec![KEY_DOWN, KEY_LEFT]);

        move_map.insert((KEY_LEFT, KEY_LEFT), vec![]);
        move_map.insert((KEY_LEFT, KEY_A), vec![KEY_RIGHT, KEY_RIGHT, KEY_UP]);
        move_map.insert((KEY_LEFT, KEY_UP), vec![KEY_RIGHT, KEY_UP]);
        move_map.insert((KEY_LEFT, KEY_DOWN), vec![KEY_RIGHT]);

        move_map.insert((KEY_RIGHT, KEY_RIGHT), vec![]);
        move_map.insert((KEY_RIGHT, KEY_A), vec![KEY_UP]);
        move_map.insert((KEY_RIGHT, KEY_UP), vec![KEY_LEFT, KEY_UP]);
        move_map.insert((KEY_RIGHT, KEY_DOWN), vec![KEY_LEFT]);

        move_map.insert((KEY_DOWN, KEY_DOWN), vec![]);
        move_map.insert((KEY_DOWN, KEY_A), vec![KEY_RIGHT, KEY_UP]);
        move_map.insert((KEY_DOWN, KEY_LEFT), vec![KEY_LEFT]);
        move_map.insert((KEY_DOWN, KEY_RIGHT), vec![KEY_RIGHT]);

        //// cursor keyboard

        move_map.insert((KEY_A, KEY_A), vec![]);
        move_map.insert((KEY_A, KEY_0), vec![KEY_LEFT]);
        move_map.insert((KEY_A, KEY_1), vec![KEY_UP, KEY_LEFT, KEY_LEFT]);
        move_map.insert((KEY_A, KEY_3), vec![KEY_UP]);
        move_map.insert((KEY_A, KEY_4), vec![KEY_UP, KEY_UP, KEY_LEFT, KEY_LEFT]);
        move_map.insert((KEY_A, KEY_9), vec![KEY_UP, KEY_UP, KEY_UP]);

        move_map.insert((KEY_0, KEY_A), vec![KEY_RIGHT]);
        move_map.insert((KEY_0, KEY_2), vec![KEY_UP]);
        move_map.insert((KEY_0, KEY_8), vec![KEY_UP, KEY_UP, KEY_UP]);

        move_map.insert((KEY_1, KEY_A), vec![KEY_RIGHT, KEY_RIGHT, KEY_DOWN]);
        move_map.insert((KEY_1, KEY_2), vec![KEY_RIGHT]);
        move_map.insert((KEY_1, KEY_7), vec![KEY_UP, KEY_UP]);

        move_map.insert((KEY_2, KEY_9), vec![KEY_RIGHT, KEY_UP, KEY_UP]);

        move_map.insert((KEY_3, KEY_A), vec![KEY_DOWN]);
        move_map.insert((KEY_3, KEY_4), vec![KEY_LEFT, KEY_LEFT, KEY_UP]);
        move_map.insert((KEY_3, KEY_7), vec![KEY_LEFT, KEY_LEFT, KEY_UP, KEY_UP]);

        move_map.insert((KEY_4, KEY_0), vec![KEY_RIGHT, KEY_DOWN, KEY_DOWN]);
        move_map.insert((KEY_4, KEY_1), vec![KEY_DOWN]);
        move_map.insert((KEY_4, KEY_5), vec![KEY_RIGHT]);
        move_map.insert((KEY_4, KEY_6), vec![KEY_RIGHT, KEY_RIGHT]);

        move_map.insert((KEY_5, KEY_6), vec![KEY_RIGHT]);

        move_map.insert((KEY_6, KEY_A), vec![KEY_DOWN, KEY_DOWN]);
        move_map.insert((KEY_6, KEY_3), vec![KEY_DOWN]);

        move_map.insert((KEY_7, KEY_9), vec![KEY_RIGHT, KEY_RIGHT]);

        move_map.insert((KEY_8, KEY_0), vec![KEY_DOWN, KEY_DOWN, KEY_DOWN]);
        move_map.insert((KEY_8, KEY_3), vec![KEY_DOWN, KEY_DOWN, KEY_RIGHT]);

        move_map.insert((KEY_9, KEY_A), vec![KEY_DOWN, KEY_DOWN, KEY_DOWN]);
        move_map.insert((KEY_9, KEY_8), vec![KEY_LEFT]);

        MoveSolver {
            move_map,
            current_key: KEY_A,
        }
    }

    fn get_move_to(&mut self, to: char) -> Vec<char> {
        let key = (self.current_key, to);
        if let Some(moves) = self.move_map.get(&key) {
            self.current_key = to;
            let mut moves = moves.clone();
            moves.push(KEY_A);
            moves
        } else {
            panic!("Invalid move from {:?} to {:?}", self.current_key, to)
        }
    }
}

pub fn day21() {
    let lines = io::read_lines("./src/day21/21.data").unwrap();

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut result = 0;
    // println!("Day21 part 1: {:?}", result);

    let mut numeric_solver = MoveSolver::new();
    let mut cursor_1_solver = MoveSolver::new();
    let mut cursor_2_solver = MoveSolver::new();
    for line in lines {
        print!("{}: ", line);
        let mut all_moves = "".to_string();
        for c in line.chars() {
            let moves = numeric_solver.get_move_to(c);
            for m in moves {
                // println!("{}", m);
                let moves = cursor_2_solver.get_move_to(m);
                for m in moves {
                    let moves = cursor_1_solver.get_move_to(m);
                    for m in moves {
                        print!("{}", m);
                        all_moves.push(m);
                    }
                }
            }
        }
        println!();
        let complexify = get_number(line) * all_moves.len() as i64;
        result += complexify;
    }

    println!("Day21 part 1: {:?}", result);
}

fn part2(lines: &Vec<String>) {
    let mut result = 0;

    let mut numeric_solver = MoveSolver::new();
    let mut cursor_solvers = Vec::new();
    for _ in 0..25 {
        cursor_solvers.push(MoveSolver::new());
    }
    let len_cursor_solvers = cursor_solvers.len();
    for line in lines {
        println!("{}: ", line);
        let mut all_moves = "".to_string();

        for c in line.chars() {
            let mut moves = numeric_solver.get_move_to(c);

            for (index, solver) in cursor_solvers.iter_mut().enumerate() {
                let mut all_next_moves = Vec::new();
                for m in moves {
                    let next_moves = solver.get_move_to(m);
                    if index == len_cursor_solvers - 1 {
                        for end_move in next_moves.iter() {
                            all_moves.push(*end_move);
                        }
                    }
                    all_next_moves.extend(next_moves);
                }
                moves = all_next_moves;
            }
        }
        // println!("{}", all_moves);
        let complexify = get_number(line) * all_moves.len() as i64;
        result += complexify;
    }

    println!("Day21 part 2: {:?}", result);
}

fn get_number(s: &str) -> i64 {
    s.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
