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

trait ActionCalled {
    fn action(&mut self, action: char);
    fn call_action(&mut self, action: char);
}

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

        //// cursor keyboard

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

struct NumericKeypad {
    current_key: char,
    action_map: HashMap<(char, char), char>,
    on_action_callback: Option<Box<dyn ActionCalled>>,
}

impl ActionCalled for NumericKeypad {
    fn call_action(&mut self, action: char) {
        if let Some(callback) = &mut self.on_action_callback {
            callback.call_action(action);
        }
    }

    fn action(&mut self, action: char) {
        let key = (self.current_key, action);
        if let Some(next_key) = self.action_map.get(&key) {
            self.current_key = *next_key;
        } else {
            panic!("Invalid action {:?}", action);
        }
    }
}

impl NumericKeypad {
    fn new() -> NumericKeypad {
        /// Action map
        let mut action_map = HashMap::new();
        action_map.insert((KEY_A, KEY_UP), KEY_3);
        action_map.insert((KEY_A, KEY_LEFT), KEY_0);
        action_map.insert((KEY_0, KEY_RIGHT), KEY_A);
        action_map.insert((KEY_0, KEY_UP), KEY_2);
        action_map.insert((KEY_1, KEY_UP), KEY_4);
        action_map.insert((KEY_1, KEY_RIGHT), KEY_2);
        action_map.insert((KEY_2, KEY_LEFT), KEY_1);
        action_map.insert((KEY_2, KEY_UP), KEY_5);
        action_map.insert((KEY_2, KEY_RIGHT), KEY_3);
        action_map.insert((KEY_2, KEY_DOWN), KEY_0);
        action_map.insert((KEY_3, KEY_LEFT), KEY_2);
        action_map.insert((KEY_3, KEY_UP), KEY_6);
        action_map.insert((KEY_3, KEY_DOWN), KEY_A);
        action_map.insert((KEY_4, KEY_UP), KEY_7);
        action_map.insert((KEY_4, KEY_RIGHT), KEY_5);
        action_map.insert((KEY_4, KEY_DOWN), KEY_1);
        action_map.insert((KEY_5, KEY_LEFT), KEY_4);
        action_map.insert((KEY_5, KEY_DOWN), KEY_2);
        action_map.insert((KEY_5, KEY_RIGHT), KEY_6);
        action_map.insert((KEY_5, KEY_UP), KEY_8);
        action_map.insert((KEY_6, KEY_UP), KEY_9);
        action_map.insert((KEY_6, KEY_LEFT), KEY_5);
        action_map.insert((KEY_6, KEY_DOWN), KEY_3);
        action_map.insert((KEY_7, KEY_DOWN), KEY_4);
        action_map.insert((KEY_7, KEY_RIGHT), KEY_8);
        action_map.insert((KEY_8, KEY_LEFT), KEY_7);
        action_map.insert((KEY_8, KEY_DOWN), KEY_5);
        action_map.insert((KEY_8, KEY_RIGHT), KEY_9);
        action_map.insert((KEY_9, KEY_LEFT), KEY_8);
        action_map.insert((KEY_9, KEY_DOWN), KEY_6);

        NumericKeypad {
            current_key: KEY_A,
            action_map,
            on_action_callback: None,
        }
    }
}

struct CursorKeypad {
    name: String,
    current_key: char,
    action_map: HashMap<(char, char), char>,
    on_action_callback: Option<Box<dyn ActionCalled>>,
}

impl ActionCalled for CursorKeypad {
    fn call_action(&mut self, action: char) {
        if let Some(callback) = &mut self.on_action_callback {
            callback.action(action);
        }
    }

    fn action(&mut self, action: char) {
        println!("action called: {}: {:?}", self.name, action);
        if action == KEY_A {
            self.call_action(self.current_key);
        } else {
            let key = (self.current_key, action);
            if let Some(next_key) = self.action_map.get(&key) {
                self.current_key = *next_key;
            } else {
                panic!("Invalid action {:?}", action);
            }
        }
    }
}

impl CursorKeypad {
    fn new(name: &str) -> CursorKeypad {
        /// Action map
        let mut action_map = HashMap::new();
        action_map.insert((KEY_A, KEY_LEFT), KEY_UP);
        action_map.insert((KEY_A, KEY_DOWN), KEY_RIGHT);
        action_map.insert((KEY_UP, KEY_DOWN), KEY_DOWN);
        action_map.insert((KEY_UP, KEY_RIGHT), KEY_A);
        action_map.insert((KEY_DOWN, KEY_UP), KEY_UP);
        action_map.insert((KEY_DOWN, KEY_LEFT), KEY_LEFT);
        action_map.insert((KEY_DOWN, KEY_RIGHT), KEY_RIGHT);
        action_map.insert((KEY_LEFT, KEY_RIGHT), KEY_DOWN);

        CursorKeypad {
            name: name.to_string(),
            current_key: KEY_A,
            action_map,
            on_action_callback: None,
        }
    }
}

struct Machine {
    numeric_keypad: NumericKeypad,
    cursor_1_keypad: CursorKeypad,
    cursor_2_keypad: CursorKeypad,
}

impl Machine {
    fn new() -> Machine {
        let numeric_keypad = NumericKeypad::new();
        let cursor_1_keypad = CursorKeypad::new("1");
        let cursor_2_keypad = CursorKeypad::new("2");

        Machine {
            cursor_1_keypad,
            cursor_2_keypad,
            numeric_keypad,
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
        let mut cursor_1_keypad = CursorKeypad::new("1");
        let mut cursor_2_keypad = CursorKeypad::new("2");

        cursor_1_keypad.on_action_callback = Some(Box::new(numeric_keypad));
        // cursor_2_keypad.on_action_callback = Some(Box::new(cursor_1_keypad));

        let mut numeric_solver = MoveSolver::new();
        let mut cursor_1_solver = MoveSolver::new();
        let mut cursor_2_solver = MoveSolver::new();

        // for c in line.chars() {
        //     let moves = numeric_solver.get_move_to(c);
        //     for m in moves {
        //         cursor_1_keypad.action(m);
        //         println!(
        //             "{} => {}/{}/{}",
        //             m,
        //             cursor_2_keypad.current_key,
        //             cursor_1_keypad.current_key,
        //             numeric_keypad.current_key
        //         );
        //     }
        // }

        cursor_2_keypad.action(KEY_LEFT);
        cursor_2_keypad.action(KEY_A);

        // for c in line.chars() {
        //     let moves = numeric_keypad.get_move_to(c);
        //     for m in moves {
        //         // println!("{}", m);
        //         let moves = box1.get_move_to(m);
        //         for m in moves {
        //             let moves = cursor_2_keypad.get_move_to(m);
        //             for m in moves {
        //                 //     println!(
        //                 //         "{} => {}/{}/{}",
        //                 //         m,
        //                 //         cursor_2_keypad.current_key,
        //                 //         cursor_1_keypad.current_key,
        //                 //         numeric_keypad.current_key
        //                 //     );
        //                 // }

        //                 // print!("{}", m);
        //                 cursor_2_keypad.action(m);
        //             }
        //         }
        //     }
        // }
        println!();
        break;
    }
}
