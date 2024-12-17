use itertools::Itertools;

use crate::util::io;

#[derive(Debug)]
struct Computer {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,

    instruction_pointer: i32,

    program: Vec<i32>,
}

const OPCODE_ADV: i32 = 0;
const OPCODE_BXL: i32 = 1;
const OPCODE_BST: i32 = 2;
const OPCODE_JNZ: i32 = 3;
const OPCODE_BXC: i32 = 4;
const OPCODE_OUT: i32 = 5;
const OPCODE_BDV: i32 = 6;
const OPCODE_CDV: i32 = 7;

impl Computer {
    fn from(lines: &[String]) -> Computer {
        fn get_register_value(line: &str) -> i32 {
            let value: i32 = line
                .split(":")
                .collect_vec()
                .get(1)
                .unwrap()
                .trim()
                .parse()
                .unwrap();

            value
        }

        let reg_a = get_register_value(&lines[0]);
        let reg_b = get_register_value(&lines[1]);
        let reg_c = get_register_value(&lines[2]);

        let programme = lines[4]
            .split(":")
            .collect_vec()
            .get(1)
            .unwrap()
            .trim()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec();

        Computer {
            reg_a,
            reg_b,
            reg_c,
            instruction_pointer: 0,
            program: programme,
        }
    }

    fn run(&mut self) -> Vec<i32> {
        let mut output = vec![];
        while self.instruction_pointer < self.program.len() as i32 {
            let opcode = self.program[self.instruction_pointer as usize];
            self.instruction_pointer += 1;
            let operand = self.program[self.instruction_pointer as usize];
            self.instruction_pointer += 1;

            match opcode {
                OPCODE_ADV => {
                    let value = self.get_combo_operand_value(operand);
                    let result = self.reg_a / 2_i32.pow(value as u32);
                    self.reg_a = result;
                }

                OPCODE_BXL => {
                    let value = operand;
                    let result = self.reg_b ^ value;
                    self.reg_b = result;
                }

                OPCODE_BST => {
                    let value = self.get_combo_operand_value(operand);
                    let result = value % 8;
                    self.reg_b = result;
                }

                OPCODE_JNZ => {
                    if self.reg_a != 0 {
                        let value = operand;
                        self.instruction_pointer = value;
                    }
                }

                OPCODE_BXC => {
                    // let value = self.get_operand_value(operand);
                    let result = self.reg_b ^ self.reg_c;
                    self.reg_b = result;
                }

                OPCODE_OUT => {
                    let value = self.get_combo_operand_value(operand);
                    let result = value % 8;
                    output.push(result);
                }

                OPCODE_BDV => {
                    let value = self.get_combo_operand_value(operand);
                    let result = self.reg_a / 2_i32.pow(value as u32);
                    self.reg_b = result;
                }

                OPCODE_CDV => {
                    let value = self.get_combo_operand_value(operand);
                    let result = self.reg_a / 2_i32.pow(value as u32);
                    self.reg_c = result;
                }

                _ => panic!("Invalid opcode {}", opcode),
            }
        }
        output
    }

    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.
    fn get_combo_operand_value(&self, operand: i32) -> i32 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo operand {}", operand),
        }
    }
}

pub fn day17() {
    let lines = io::read_lines("./src/day17/17.data").unwrap();

    let mut computer = Computer::from(&lines);

    let output = computer.run();
    // let output = output.iter().map(|x| x.to_string()).collect_vec().join("");
    // println!("Day17 part 1: {:?}", output);

    println!("debug: {:?} out:{:?}", computer, output);

    let result = 0;
}
