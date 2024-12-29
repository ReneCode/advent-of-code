use core::num;
use std::{collections::HashMap, hash::Hash};

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

struct NumericKeypad {
    current_key: char,
    move_map: HashMap<(char, char), Vec<char>>,
}

impl NumericKeypad {
    fn new() -> NumericKeypad {
        let mut move_map = HashMap::new();
        move_map.insert((KEY_A, KEY_A), vec![]);
        move_map.insert((KEY_A, KEY_0), vec![KEY_LEFT]);
        move_map.insert((KEY_A, KEY_1), vec![KEY_UP, KEY_LEFT, KEY_LEFT]);
        move_map.insert((KEY_A, KEY_3), vec![KEY_UP]);
        move_map.insert((KEY_A, KEY_4), vec![KEY_UP, KEY_UP, KEY_LEFT, KEY_LEFT]);
        move_map.insert((KEY_A, KEY_9), vec![KEY_UP, KEY_UP, KEY_UP]);

        move_map.insert((KEY_0, KEY_A), vec![KEY_RIGHT, KEY_UP, KEY_UP, KEY_UP]);
        move_map.insert((KEY_0, KEY_2), vec![KEY_UP]);

        move_map.insert((KEY_1, KEY_7), vec![KEY_UP, KEY_UP]);

        move_map.insert((KEY_2, KEY_9), vec![KEY_RIGHT, KEY_UP, KEY_UP]);

        move_map.insert((KEY_3, KEY_7), vec![KEY_UP, KEY_UP, KEY_LEFT, KEY_LEFT]);

        move_map.insert((KEY_4, KEY_5), vec![KEY_RIGHT]);

        move_map.insert((KEY_5, KEY_6), vec![KEY_RIGHT]);

        move_map.insert((KEY_6, KEY_A), vec![KEY_DOWN, KEY_DOWN]);

        move_map.insert((KEY_7, KEY_9), vec![KEY_RIGHT, KEY_RIGHT]);

        move_map.insert((KEY_8, KEY_0), vec![KEY_DOWN, KEY_DOWN, KEY_DOWN]);

        move_map.insert((KEY_9, KEY_A), vec![KEY_DOWN, KEY_DOWN, KEY_DOWN]);
        move_map.insert((KEY_9, KEY_8), vec![KEY_LEFT]);

        NumericKeypad {
            current_key: KEY_A,
            move_map,
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

struct CursorKeypad {
    current_key: char,
    move_map: HashMap<(char, char), Vec<char>>,
}

impl CursorKeypad {
    fn new() -> CursorKeypad {
        let mut move_map = HashMap::new();
        move_map.insert((KEY_A, KEY_LEFT), vec![KEY_DOWN, KEY_LEFT, KEY_LEFT]);
        move_map.insert((KEY_A, KEY_UP), vec![KEY_LEFT]);
        move_map.insert((KEY_A, KEY_RIGHT), vec![KEY_DOWN]);
        move_map.insert((KEY_A, KEY_DOWN), vec![KEY_LEFT, KEY_DOWN]);
        move_map.insert((KEY_A, KEY_A), vec![]);

        move_map.insert((KEY_UP, KEY_UP), vec![]);
        move_map.insert((KEY_UP, KEY_RIGHT), vec![KEY_RIGHT, KEY_DOWN]);
        move_map.insert((KEY_UP, KEY_A), vec![KEY_RIGHT]);
        move_map.insert((KEY_UP, KEY_LEFT), vec![KEY_DOWN, KEY_LEFT]);

        move_map.insert((KEY_LEFT, KEY_LEFT), vec![]);
        move_map.insert((KEY_LEFT, KEY_A), vec![KEY_RIGHT, KEY_RIGHT, KEY_UP]);
        move_map.insert((KEY_LEFT, KEY_UP), vec![KEY_RIGHT, KEY_UP]);
        move_map.insert((KEY_LEFT, KEY_DOWN), vec![KEY_RIGHT]);

        move_map.insert((KEY_RIGHT, KEY_RIGHT), vec![]);
        move_map.insert((KEY_RIGHT, KEY_A), vec![KEY_UP]);
        move_map.insert((KEY_RIGHT, KEY_UP), vec![KEY_LEFT, KEY_UP]);

        move_map.insert((KEY_DOWN, KEY_DOWN), vec![]);
        move_map.insert((KEY_DOWN, KEY_A), vec![KEY_RIGHT, KEY_UP]);
        move_map.insert((KEY_DOWN, KEY_LEFT), vec![KEY_LEFT]);

        CursorKeypad {
            current_key: KEY_A,
            move_map,
        }
    }

    fn get_move_to(&mut self, to: char) -> Vec<char> {
        let key = (self.current_key, to);
        if let Some(moves) = self.move_map.get(&key) {
            self.current_key = to;
            let mut moves = moves.clone();
            moves.push(KEY_A);
            return moves;
        }
        panic!(
            "Invalid cursor move from {:?} to {:?}",
            self.current_key, to
        )
    }

    fn action(&mut self, action: char) {
        match action {
            KEY_UP => {
                self.current_key = KEY_UP;
            }
            KEY_DOWN => {
                self.current_key = KEY_DOWN;
            }
            KEY_LEFT => {
                self.current_key = KEY_LEFT;
            }
            KEY_RIGHT => {
                self.current_key = KEY_RIGHT;
            }
            _ => {
                panic!("Invalid action {:?}", action);
            }
        }
    }
}

pub fn day21() {
    let lines = io::read_lines("./src/day21/21.data").unwrap();

    part1(&lines);
}

fn part1(lines: &Vec<String>) {
    let result = 0;
    // println!("Day21 part 1: {:?}", result);

    for line in lines {
        println!("Line: {:?} : ", line);
        let mut numeric_keypad = NumericKeypad::new();
        let mut cursor_1_keypad = CursorKeypad::new();
        let mut cursor_2_keypad = CursorKeypad::new();
        for c in line.chars() {
            let moves = numeric_keypad.get_move_to(c);
            for m in moves {
                // println!("{}", m);
                let moves = cursor_1_keypad.get_move_to(m);
                for m in moves {
                    let moves = cursor_2_keypad.get_move_to(m);
                    for m in moves {
                        //     println!(
                        //         "{} => {}/{}/{}",
                        //         m,
                        //         cursor_2_keypad.current_key,
                        //         cursor_1_keypad.current_key,
                        //         numeric_keypad.current_key
                        //     );
                        // }

                        print!("{}", m);
                    }
                }
            }
        }
        println!();
        // break;
    }
}
