// day08

use std::collections::HashSet;

use crate::util::io;

#[derive(Debug)]
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

    let mut cpu = CPU {
        accumulator: 0,
        program_counter: 0,
        instructions,
    };
    cpu.run_until_loop();
    println!("A: {}", cpu.acc());
}
