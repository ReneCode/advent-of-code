use crate::util::io;
use itertools::Itertools;

type Number = i64;

struct Equation {
    result: Number,
    parts: Vec<Number>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
enum Operation {
    Add,
    Multiply,
}

impl Equation {
    fn solvable(&self) -> bool {
        // let operations = (Operation::Add, Operation::Multiply).com
        let all_operations = self.all_operations();

        for operations in all_operations {
            let result = self.evaluate(&operations);
            if result == self.result {
                return true;
            }
        }

        false
    }

    fn all_operations(&self) -> Vec<Vec<Operation>> {
        let mut result = Vec::new();
        let gaps = self.parts.len() - 1;

        // in total there are 2^gaps possible operations
        // look at the binary representation of the number
        let max_nr: u32 = (2 as u32).pow(gaps as u32);
        for i in 0..max_nr {
            let mut operations = vec![Operation::Add; gaps];
            for pos in 0..gaps {
                if 1 & (i >> pos) == 1 {
                    operations[pos] = Operation::Multiply;
                }
            }
            result.push(operations);
        }

        result
    }

    fn evaluate(&self, operations: &[Operation]) -> Number {
        let mut result = self.parts[0];
        for i in 0..operations.len() {
            let val = self.parts[i + 1];
            result = match operations[i] {
                Operation::Add => result + val,
                Operation::Multiply => result * val,
            }
        }
        result
    }
}

pub fn day07() {
    let lines = io::read_lines("./src/day07/07.data").unwrap();

    let equations = lines
        .iter()
        .map(|line| {
            let tok = line.split(":").map(|s| s.trim()).collect_vec();
            let result: Number = tok[0].parse().unwrap();
            let parts: Vec<Number> = tok[1].split(" ").map(|s| s.parse().unwrap()).collect_vec();
            Equation { result, parts }
        })
        .collect_vec();

    let result = equations
        .iter()
        .filter(|eq| eq.solvable())
        .map(|eq| eq.result)
        .sum::<Number>();
    println!("Day 07: Part 1) = {:?}", result);
}
