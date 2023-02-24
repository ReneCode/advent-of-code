use std::{collections::HashMap, usize};

use util::point::Point;

const CODE_STOP: i64 = 99;
const CODE_ADD: i64 = 1;
const CODE_MULTIPLY: i64 = 2;
const CODE_SAVE_INPUT: i64 = 3;
const CODE_OUTPUT: i64 = 4;
const CODE_JUMP_IF_TRUE: i64 = 5;
const CODE_JUMP_IF_FALSE: i64 = 6;
const CODE_CMP_IF_LT: i64 = 7;
const CODE_CMP_IF_EQ: i64 = 8;
const CODE_RELATIVE_BASE: i64 = 9;

const MODE_POSITION: i64 = 0;
const MODE_IMMEDIATE: i64 = 1;
const MODE_RELATIVE: i64 = 2;

const COLOR_BLACK: i64 = 0;
const COLOR_WHITE: i64 = 1;

const TURN_LEFT: i64 = 0;
const TURN_RIGHT: i64 = 1;

enum OutputType {
    Color,
    Direction,
}

#[derive(Debug)]
enum StepResult {
    Ok,
    Output(i64),
    Stop,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Program = Vec<i64>;

struct Area {
    output_type: OutputType,
    tiles: HashMap<Point, i64>,
    current_point: Point,
    current_direction: Direction,
    count_set_color: i32,
}

impl Area {
    fn new() -> Self {
        Area {
            output_type: OutputType::Color,
            tiles: HashMap::new(),
            current_point: Point::new(0, 0),
            current_direction: Direction::Up,
            count_set_color: 0,
        }
    }

    fn take_output(&mut self, value: i64) -> Option<i64> {
        let mut result = None;
        self.output_type = match self.output_type {
            OutputType::Color => {
                self.set_color(value);
                OutputType::Direction
            }

            OutputType::Direction => {
                let color = self.set_turn_direction(value);
                result = Some(color);
                OutputType::Color
            }
        };
        result
    }

    fn set_color(&mut self, color: i64) {
        if !self.tiles.contains_key(&self.current_point) {
            self.count_set_color += 1;
            // println!("{}", self.count_set_color);
        }
        self.tiles.insert(self.current_point.clone(), color);
        // println!("set color {}", color);
    }

    fn set_turn_direction(&mut self, turn: i64) -> i64 {
        // println!("turn direction {}", turn);
        match turn {
            TURN_LEFT => {
                self.current_direction = match self.current_direction {
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                }
            }
            TURN_RIGHT => {
                self.current_direction = match self.current_direction {
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                }
            }
            _ => {
                panic!("bad direction {}", turn);
            }
        }
        self.current_point = match self.current_direction {
            Direction::Up => self.current_point.add(&Point::new(0, 1)),
            Direction::Down => self.current_point.add(&Point::new(0, -1)),
            Direction::Left => self.current_point.add(&Point::new(-1, 0)),
            Direction::Right => self.current_point.add(&Point::new(1, 0)),
        };

        self.get_color()
    }

    fn get_color(&self) -> i64 {
        if let Some(tile) = self.tiles.get(&self.current_point) {
            let current_color = *tile;
            // println!(
            //     "current color on {:?} {}",
            //     self.current_point, current_color
            // );
            return current_color;
        } else {
            COLOR_BLACK
        }
    }

    fn print_tiles(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for pt in self.tiles.keys() {
            min_x = min_x.min(pt.x);
            max_x = max_x.max(pt.x);
            min_y = min_y.min(pt.y);
            max_y = max_y.max(pt.y);
        }
        for y in (min_y..=max_y).rev() {
            let mut line = String::from("");
            for x in (min_x..=max_x) {
                let key = Point::new(x, y);
                if let Some(color) = self.tiles.get(&key) {
                    if *color == COLOR_WHITE {
                        line.push('#');
                    } else {
                        line.push(' ');
                    }
                }
            }
            println!("{}", line);
        }
    }
}

enum Parameter {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl Parameter {
    pub fn new(mode: i64, value: i64) -> Self {
        match mode {
            MODE_IMMEDIATE => Parameter::Immediate(value),
            MODE_POSITION => Parameter::Position(value),
            MODE_RELATIVE => Parameter::Relative(value),
            _ => panic!("bad mode: {mode}"),
        }
    }
}

enum Instruction {
    Stop,
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    SaveInput(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    CompareIfLessThan(Parameter, Parameter, Parameter),
    CompareIfEqual(Parameter, Parameter, Parameter),
    AdjustRelativeBase(Parameter),
}

impl Instruction {
    pub fn execute(&self, amplifier: &mut Amplifier) -> StepResult {
        let result = match self {
            Self::Stop => StepResult::Stop,
            Self::Add(a, b, c) => {
                let val_a = amplifier.read_value(a);
                let val_b = amplifier.read_value(b);
                amplifier.write_value(c, val_a + val_b);
                StepResult::Ok
            }
            Self::Multiply(a, b, c) => {
                let val_a = amplifier.read_value(a);
                let val_b = amplifier.read_value(b);
                amplifier.write_value(c, val_a * val_b);
                StepResult::Ok
            }
            Self::SaveInput(a) => {
                let val = amplifier.inputs.remove(0);
                amplifier.write_value(a, val);
                StepResult::Ok
            }
            Self::Output(a) => {
                let val = amplifier.read_value(a);
                StepResult::Output(val)
            }
            Self::JumpIfTrue(a, b) => {
                let val_a = amplifier.read_value(a);
                let val_b = amplifier.read_value(b);
                if val_a != 0 {
                    amplifier.address = val_b as usize;
                }
                StepResult::Ok
            }
            Self::JumpIfFalse(a, b) => {
                let val_a = amplifier.read_value(a);
                let val_b = amplifier.read_value(b);
                if val_a == 0 {
                    amplifier.address = val_b as usize;
                }
                StepResult::Ok
            }
            Self::CompareIfLessThan(a, b, c) => {
                let val_a = amplifier.read_value(a);
                let val_b = amplifier.read_value(b);
                let val = if val_a < val_b { 1 } else { 0 };
                amplifier.write_value(c, val);
                StepResult::Ok
            }
            Self::CompareIfEqual(a, b, c) => {
                let val_a = amplifier.read_value(a);
                let val_b = amplifier.read_value(b);
                let val = if val_a == val_b { 1 } else { 0 };
                amplifier.write_value(c, val);
                StepResult::Ok
            }
            Self::AdjustRelativeBase(a) => {
                let val_a = amplifier.read_value(a);
                amplifier.relative_base = ((amplifier.relative_base as i64) + val_a) as usize;
                StepResult::Ok
            } // _ => panic!("ups not handled instruction"),
        };
        result
    }
}

struct Amplifier {
    programm: Program,
    address: usize,
    relative_base: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl Amplifier {
    fn new(line: &str) -> Self {
        Amplifier {
            programm: create_program(line),
            address: 0,
            relative_base: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    fn run(&mut self) -> StepResult {
        loop {
            let result = self.step();
            match result {
                StepResult::Stop => break result,
                StepResult::Output(val) => {
                    self.outputs.push(val);
                }
                StepResult::Ok => {}
            }
        }
    }

    fn walk_robot(&mut self, area: &mut Area) {
        loop {
            let result = self.step();
            match result {
                StepResult::Stop => break,
                StepResult::Output(val) => {
                    if let Some(input) = area.take_output(val) {
                        self.add_input(input);
                    }
                }
                StepResult::Ok => {}
            }
        }
    }

    fn read(&mut self) -> i64 {
        let code = self.programm[self.address];
        self.address += 1;
        code
    }

    fn step(&mut self) -> StepResult {
        let code = self.read();
        let opcode = code % 100;
        let mode_a = (code / 100) % 10;
        let mode_b = (code / 1000) % 10;
        let mode_c = (code / 10000) % 10;
        let instruction = match opcode {
            CODE_STOP => Instruction::Stop,
            CODE_ADD => Instruction::Add(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_MULTIPLY => Instruction::Multiply(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_SAVE_INPUT => Instruction::SaveInput(Parameter::new(mode_a, self.read())),
            CODE_OUTPUT => Instruction::Output(Parameter::new(mode_a, self.read())),

            CODE_JUMP_IF_TRUE => Instruction::JumpIfTrue(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
            ),
            CODE_JUMP_IF_FALSE => Instruction::JumpIfFalse(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
            ),
            CODE_CMP_IF_EQ => Instruction::CompareIfEqual(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_CMP_IF_LT => Instruction::CompareIfLessThan(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_RELATIVE_BASE => {
                Instruction::AdjustRelativeBase(Parameter::new(mode_a, self.read()))
            }
            _ => panic!("bad opcode {opcode}"),
        };

        instruction.execute(self)
    }

    fn read_value(&mut self, parameter: &Parameter) -> i64 {
        match parameter {
            Parameter::Immediate(address) => {
                return *address;
            }
            Parameter::Position(address) => {
                let adr = *address as usize;
                self.resize_if_necessary(adr);
                return self.programm[adr];
            }
            Parameter::Relative(address) => {
                let adr = (*address as i64 + self.relative_base as i64) as usize;
                self.resize_if_necessary(adr);
                return self.programm[adr];
            }
        };
    }

    fn write_value(&mut self, parameter: &Parameter, value: i64) {
        match parameter {
            Parameter::Immediate(_address) => {
                panic!("canot write in immediate mode")
            }
            Parameter::Position(address) => {
                let adr = *address as usize;
                self.resize_if_necessary(adr);
                self.programm[adr] = value
            }
            Parameter::Relative(address) => {
                let adr = (*address as i64 + self.relative_base as i64) as usize;
                self.resize_if_necessary(adr);
                self.programm[adr] = value
            }
        }
    }

    fn resize_if_necessary(&mut self, address: usize) {
        if address >= self.programm.len() {
            self.programm.resize(address + 1, 0);
        }
    }
}

fn main() {
    println!("Hello, day11!");

    if let Some(input) = util::io::get_lines("./11.data") {
        if let Some(line) = input.get(0) {
            part_1(line.as_str());
            part_2(line.as_str());
        }
    }
}

fn create_program(line: &str) -> Program {
    let program: Program = line
        .split(",")
        .map(|s| {
            if let Ok(val) = s.parse::<i64>() {
                val
            } else {
                0
            }
        })
        .collect();
    program
}

fn part_1(line: &str) {
    let mut amp = Amplifier::new(line);
    amp.add_input(COLOR_BLACK);
    let mut area = Area::new();
    let a = amp.walk_robot(&mut area);
    println!("part-1 count set color: {:?}", area.count_set_color);
}

fn part_2(line: &str) {
    let mut amp = Amplifier::new(line);
    amp.add_input(COLOR_WHITE);
    let mut area = Area::new();
    amp.walk_robot(&mut area);
    area.print_tiles();
    // println!("part-2 count set color: {:?}", area.count_set_color);
}

#[test]
fn test_2_1() {
    let line = "1,0,0,0,99";
    let mut amp = Amplifier::new(line);
    let a = amp.run();
    let expect = vec![2, 0, 0, 0, 99];
    assert_eq!(expect, amp.programm);
}

#[test]
fn test_2_2() {
    let line = "2,3,0,3,99";
    let mut amp = Amplifier::new(line);
    let a = amp.run();
    let expect = vec![2, 3, 0, 6, 99];
    assert_eq!(expect, amp.programm);
}

#[test]
fn test_2_3() {
    let line = "2,4,4,5,99,0";
    let mut amp = Amplifier::new(line);
    let a = amp.run();
    let expect = vec![2, 4, 4, 5, 99, 9801];
    assert_eq!(expect, amp.programm);
}

#[test]
fn test_2_4() {
    let line = "1,1,1,4,99,5,6,0,99";
    let mut amp = Amplifier::new(line);
    let a = amp.run();
    let expect = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    assert_eq!(expect, amp.programm);
}

#[test]
fn test_5_equal_8() {
    let line = "3,9,8,9,10,9,4,9,99,-1,8";
    let mut amp = Amplifier::new(line);
    amp.add_input(8);
    let a = amp.run();
    let expect = vec![1];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_not_equal_8() {
    let line = "3,9,8,9,10,9,4,9,99,-1,8";
    let mut amp = Amplifier::new(line);
    amp.add_input(7);
    let a = amp.run();
    let expect = vec![0];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_less_than_8() {
    let line = "3,9,7,9,10,9,4,9,99,-1,8";
    let mut amp = Amplifier::new(line);
    amp.add_input(7);
    let _ = amp.run();
    let expect = vec![1];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_not_less_than_8() {
    let line = "3,9,7,9,10,9,4,9,99,-1,8";
    let mut amp = Amplifier::new(line);
    amp.add_input(8);
    let _ = amp.run();
    let expect = vec![0];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_equal_8_immediate_mode() {
    let line = "3,3,1108,-1,8,3,4,3,99";
    let mut amp = Amplifier::new(line);
    amp.add_input(8);
    let _ = amp.run();
    let expect = vec![1];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_not_equal_8_immediate_mode() {
    let line = "3,3,1108,-1,8,3,4,3,99";
    let inputs = vec![9];
    let mut amp = Amplifier::new(line);
    amp.add_input(9);
    let _ = amp.run();
    let expect = vec![0];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_less_than_8_immediate_mode() {
    let line = "3,3,1107,-1,8,3,4,3,99";
    let inputs = vec![6];
    let mut amp = Amplifier::new(line);
    amp.add_input(6);
    let _ = amp.run();
    let expect = vec![1];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_not_less_than_8_immediate_mode() {
    let line = "3,3,1107,-1,8,3,4,3,99";
    let mut amp = Amplifier::new(line);
    amp.add_input(9);
    let _ = amp.run();
    let expect = vec![0];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_999_if_below_8() {
    let line = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mut amp = Amplifier::new(line);
    amp.add_input(7);
    let _ = amp.run();
    let expect = vec![999];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_1000_if_equal_8() {
    let line = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mut amp = Amplifier::new(line);
    amp.add_input(8);
    let _ = amp.run();
    let expect = vec![1000];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_5_1001_if_greater_8() {
    let line = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mut amp = Amplifier::new(line);
    amp.add_input(9);
    let _ = amp.run();
    let expect = vec![1001];
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_9_copy_itself() {
    let line = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut amp = Amplifier::new(line);
    let _ = amp.run();
    let expect = create_program(line);
    assert_eq!(expect, amp.outputs);
}

#[test]
fn test_amplifier_9_output_16_digit_number() {
    let line = "1102,34915192,34915192,7,4,7,99,0";
    let mut amp = Amplifier::new(line);
    if let StepResult::Output(output) = amp.run() {
        assert_eq!(16, format!("{}", output).len());
    }
}

#[test]
fn test_9_output_large_number() {
    let line = "104,1125899906842624,99";
    let mut amp = Amplifier::new(line);
    let _ = amp.run();
    let expect = vec![1125899906842624];
    assert_eq!(expect, amp.outputs);
}

// --------
