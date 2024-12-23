use itertools::Itertools;

use crate::util::io;

#[derive(Debug)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,

    instruction_pointer: i64,

    program: Vec<i64>,
}

const OPCODE_ADV: i64 = 0;
const OPCODE_BXL: i64 = 1;
const OPCODE_BST: i64 = 2;
const OPCODE_JNZ: i64 = 3;
const OPCODE_BXC: i64 = 4;
const OPCODE_OUT: i64 = 5;
const OPCODE_BDV: i64 = 6;
const OPCODE_CDV: i64 = 7;

impl Computer {
    fn from(lines: &[String]) -> Computer {
        fn get_register_value(line: &str) -> i64 {
            let value: i64 = line
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
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();

        Computer {
            reg_a,
            reg_b,
            reg_c,
            instruction_pointer: 0,
            program: programme,
        }
    }

    fn run(&mut self) -> Vec<i64> {
        let mut output = vec![];
        while self.instruction_pointer < self.program.len() as i64 {
            let opcode = self.program[self.instruction_pointer as usize];
            self.instruction_pointer += 1;
            let operand = self.program[self.instruction_pointer as usize];
            self.instruction_pointer += 1;

            match opcode {
                OPCODE_ADV => {
                    let value = self.get_combo_operand_value(operand);
                    let result = self.reg_a / 2_i64.pow(value as u32);
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
                    let result = self.reg_a / 2_i64.pow(value as u32);
                    self.reg_b = result;
                }

                OPCODE_CDV => {
                    let value = self.get_combo_operand_value(operand);
                    let result = self.reg_a / 2_i64.pow(value as u32);
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
    fn get_combo_operand_value(&self, operand: i64) -> i64 {
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
    let output = output.iter().map(|x| x.to_string()).collect_vec().join(",");
    println!("Day17 part 1: {:?}", output);

    let programm = computer.program.clone();

    let prog_len = programm.len();

    let mut factors = vec![];
    for i in 0..computer.program.len() as u32 {
        let mut a = 0;
        for x in 0..i as u32 {
            let exp: u32 = i - x;
            let da = i64::pow(8, exp) * factors[x as usize];
            a += da;
        }
        for idx in 0..8 {
            let out = run_with_a(&mut computer, a);
            // println!("i:{:?}/idx: {:?}:{:?} => {:?}", i, idx, programm, out);
            if vector_with_same_end(&out, &programm) {
                if i == computer.program.len() as u32 - 1 {
                    println!("Day17 part 2: {:?}", a);
                }
                factors.push(idx);
                break;
            }
            a += 1;
        }
    }
}

fn vector_with_same_end(a: &Vec<i64>, b: &Vec<i64>) -> bool {
    let a_len = a.len();

    let b_len = b.len();
    for i in 0..a_len {
        if a[a_len - i - 1] != b[b_len - i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_with_same_end() {
        let a = vec![5];
        let b = vec![1, 2, 3, 4, 5];
        assert_eq!(vector_with_same_end(&a, &b), true);

        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 3, 4, 6];
        assert_eq!(vector_with_same_end(&a, &b), false);

        let a = vec![3, 4];
        let b = vec![1, 2, 3, 4];
        assert_eq!(vector_with_same_end(&a, &b), true);

        let a = vec![1, 3, 4];
        let b = vec![1, 2, 3, 4];
        assert_eq!(vector_with_same_end(&a, &b), false);
    }
}

fn run_with_a(computer: &mut Computer, a: i64) -> Vec<i64> {
    computer.reg_a = a;
    computer.reg_b = 0;
    computer.reg_c = 0;
    computer.instruction_pointer = 0;
    computer.run()
}
