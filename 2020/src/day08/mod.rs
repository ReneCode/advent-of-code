// day08

use std::collections::HashSet;

use crate::util::io;

#[derive(Clone, Debug)]
struct Instruction {
    operation: String,
    argument: i32,
}

#[derive(Debug)]
struct CPU {
    accumulator: i32,
    program_counter: i32,
    instructions: Vec<Instruction>,
}

impl CPU {
    fn acc(&self) -> i32 {
        self.accumulator
    }

    fn step(&mut self) {
        let instruction = &self.instructions[self.program_counter as usize];
        match instruction.operation.as_str() {
            "nop" => {
                self.program_counter += 1;
            }
            "acc" => {
                self.accumulator += instruction.argument;
                self.program_counter += 1;
            }
            "jmp" => {
                self.program_counter += instruction.argument;
            }
            _ => {
                panic!("Unknown operation: {}", instruction.operation);
            }
        }
    }

    fn run_until_loop(&mut self) {
        let mut visited: HashSet<i32> = HashSet::new();

        while !visited.contains(&self.program_counter) {
            visited.insert(self.program_counter);
            self.step();
            // println!("PC:{} acc:{}", self.program_counter, self.accumulator);
        }
    }

    fn run_until_end(&mut self) -> bool {
        let mut visited: HashSet<i32> = HashSet::new();

        while !visited.contains(&self.program_counter) {
            visited.insert(self.program_counter);
            self.step();
            if self.program_counter == self.instructions.len() as i32 {
                return true;
            }
        }
        // sorry, infinite loop detected
        false
    }
}

pub fn day08() {
    let lines = io::read_lines("08.data").unwrap();

    let instructions = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let operation = parts[0].to_string();
            let argument = parts[1].parse::<i32>().unwrap();
            Instruction {
                operation,
                argument,
            }
        })
        .collect();

    part1(&instructions);

    part2(&instructions);
}

fn part2(instructions: &Vec<Instruction>) {
    for i in 0..instructions.len() {
        let mut cpu = CPU {
            accumulator: 0,
            program_counter: 0,
            instructions: instructions.clone(),
        };

        match cpu.instructions[i].operation.as_str() {
            "nop" => {
                cpu.instructions[i].operation = "jmp".to_string();
                if cpu.run_until_end() {
                    println!("B: {}", cpu.acc());
                    break;
                }
                cpu.instructions[i].operation = "nop".to_string();
            }
            "jmp" => {
                cpu.instructions[i].operation = "nop".to_string();
                if cpu.run_until_end() {
                    println!("B: {}", cpu.acc());
                    break;
                }
                cpu.instructions[i].operation = "jmp".to_string();
            }
            _ => {}
        }
    }
}

fn part1(instructions: &Vec<Instruction>) {
    let mut cpu = CPU {
        accumulator: 0,
        program_counter: 0,
        instructions: instructions.clone(),
    };

    cpu.run_until_loop();
    println!("A: {}", cpu.acc());
}
