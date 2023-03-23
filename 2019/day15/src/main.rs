use std::{collections::HashMap, usize};

use util::point::BoundingBox;
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

const MOVE_NORTH: i64 = 1;
const MOVE_SOUTH: i64 = 2;
const MOVE_WEST: i64 = 3;
const MOVE_EAST: i64 = 4;

const STATUS_HIT_WALL: i64 = 0;
const STATUS_MOVED: i64 = 1;
const STATUS_FOUND_OXYGEN_SYSTEM: i64 = 2;

const TILE_WALL: char = '#';
const TILE_FREE: char = '.';
const TILE_DROID: char = 'D';
const TILE_UNEXPLORED: char = ' ';
const TILE_OXYGEN: char = '*';

#[derive(Debug)]
enum StepResult {
    Ok,
    Output(i64),
    Stop,
}

type Program = Vec<i64>;

pub trait TakeInputOutput {
    fn read_input(&mut self) -> i64;
    fn take_output(&mut self, value: i64) -> bool;
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
    pub fn execute(
        &self,
        amplifier: &mut IntComputer,
        device: &mut impl TakeInputOutput,
    ) -> StepResult {
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
                let val = device.read_input();
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

struct IntComputer {
    programm: Program,
    address: usize,
    relative_base: usize,
}

struct DemoDevice {
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl TakeInputOutput for DemoDevice {
    fn read_input(&mut self) -> i64 {
        let val = self.inputs.remove(0);
        val
    }

    fn take_output(&mut self, value: i64) -> bool {
        self.outputs.push(value);
        false
    }
}

impl DemoDevice {
    fn new() -> Self {
        DemoDevice {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }
}

impl IntComputer {
    fn new(line: &str) -> Self {
        IntComputer {
            programm: create_program(line),
            address: 0,
            relative_base: 0,
        }
    }

    fn run(&mut self, device: &mut impl TakeInputOutput) -> StepResult {
        loop {
            let result = self.step(device);
            match result {
                StepResult::Stop => break result,
                StepResult::Output(val) => {
                    match device.take_output(val) {
                        false => {}
                        true => break result,
                    }
                    // self.outputs.push(val);
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

    fn step(&mut self, device: &mut impl TakeInputOutput) -> StepResult {
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

        instruction.execute(self, device)
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

struct RepairDroid {
    tiles: HashMap<Point, char>,
    last_positions: Vec<Point>,
}

impl RepairDroid {
    fn new() -> Self {
        let mut droid = RepairDroid {
            tiles: HashMap::new(),
            last_positions: Vec::new(),
        };
        let start_point = Point::new(0, 0);
        droid.tiles.insert(start_point.clone(), TILE_FREE);
        droid.last_positions.push(start_point);
        droid
    }

    fn print(&self) {
        let mut bounding_box = BoundingBox::new();
        for (point, _pos_type) in self.tiles.iter() {
            bounding_box.add(point);
        }
        println!();
        for y in bounding_box.y_min..=bounding_box.y_max {
            let mut line = String::new();
            for x in bounding_box.x_min..=bounding_box.x_max {
                let pt = Point::new(x, y);
                let last_pos = self.last_positions.last().unwrap();
                if last_pos.x == pt.x && last_pos.y == pt.y {
                    line.push('D')
                } else if pt.x == 0 && pt.y == 0 {
                    line.push('*')
                } else {
                    if let Some(tile) = self.tiles.get(&pt) {
                        line.push(*tile);
                    } else {
                        line.push(TILE_UNEXPLORED)
                    }
                }
            }
            println!("{}", line);
        }
    }

    fn get_tile(&self, x: i32, y: i32) -> char {
        let pt = Point::new(x, y);
        if let Some(tile) = self.tiles.get(&pt) {
            *tile
        } else {
            TILE_UNEXPLORED
        }
    }

    fn get_unexplored_neighbour(&self, pt: &Point) -> Option<(Point, i64)> {
        if self.get_tile(pt.x + 1, pt.y) == TILE_UNEXPLORED {
            return Some((Point::new(pt.x + 1, pt.y), MOVE_EAST));
        }
        if self.get_tile(pt.x - 1, pt.y) == TILE_UNEXPLORED {
            return Some((Point::new(pt.x - 1, pt.y), MOVE_WEST));
        }
        if self.get_tile(pt.x, pt.y + 1) == TILE_UNEXPLORED {
            return Some((Point::new(pt.x, pt.y + 1), MOVE_NORTH));
        }
        if self.get_tile(pt.x, pt.y - 1) == TILE_UNEXPLORED {
            return Some((Point::new(pt.x, pt.y - 1), MOVE_SOUTH));
        }

        None
    }

    fn get_unexplored_points() {}
}

impl TakeInputOutput for RepairDroid {
    fn read_input(&mut self) -> i64 {
        self.print();

        loop {
            if let Some(pt) = self.last_positions.last() {
                if let Some((next_point, next_move)) = self.get_unexplored_neighbour(pt) {
                    self.last_positions.push(next_point);
                    return next_move;
                } else {
                    // leave that pt
                    let cur_pt = pt.clone();
                    let _ = self.last_positions.pop().unwrap();
                    let prev_pt = self.last_positions.last().unwrap();
                    // go in the reverse direction
                    if prev_pt.x + 1 == cur_pt.x {
                        return MOVE_WEST;
                    } else if prev_pt.x - 1 == cur_pt.x {
                        return MOVE_EAST;
                    } else if prev_pt.y + 1 == cur_pt.y {
                        return MOVE_SOUTH;
                    } else if prev_pt.y - 1 == cur_pt.y {
                        return MOVE_NORTH;
                    } else {
                        panic!()
                    }
                    // println!("{} / {}", cur_pt, prev_pt);
                }
            }
        }
    }

    fn take_output(&mut self, value: i64) -> bool {
        match value {
            STATUS_HIT_WALL => {
                let pos = self.last_positions.pop().unwrap();
                self.tiles.insert(pos, TILE_WALL);
            }
            STATUS_MOVED => {
                let last_pos = self.last_positions.last().unwrap();
                self.tiles.insert(last_pos.clone(), TILE_FREE);
            }
            STATUS_FOUND_OXYGEN_SYSTEM => {
                let last_pos = self.last_positions.last().unwrap();
                self.tiles.insert(last_pos.clone(), TILE_OXYGEN);
                return true;
            }
            _ => {}
        }
        false
    }
}

fn main() {
    println!("Hello, day15!");

    if let Some(input) = util::io::get_lines("./15.data") {
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
    let mut computer = IntComputer::new(line);
    let mut repair_droid = RepairDroid::new();
    computer.run(&mut repair_droid);
    // -1 because of do not count the start-point
    println!("part1 steps:{}", repair_droid.last_positions.len() - 1);
}

fn part_2(line: &str) {
    let amp = IntComputer::new(line);
}
