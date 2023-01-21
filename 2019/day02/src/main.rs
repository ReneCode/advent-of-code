extern crate util;

const CODE_STOP: usize = 99;
const CODE_ADD: usize = 1;
const CODE_MULTIPLY: usize = 2;

fn main() {
    println!("Hello, day02!");
    if let Some(input) = util::io::get_lines("./02.data") {
        if let Some(line) = input.get(0) {
            println!("part-1 result: {}", work_programm(line));
        }
    }
}

fn work_programm(line: &String) -> usize {
    let mut program: Vec<usize> = line
        .split(",")
        .map(|s| {
            if let Ok(val) = s.parse::<usize>() {
                val
            } else {
                0
            }
        })
        .collect();

    // restore the gravity
    program[1] = 12;
    program[2] = 2;

    let mut idx: usize = 0;
    loop {
        let opcode = program[idx];
        match opcode {
            CODE_STOP => break,
            CODE_ADD => {
                let a_adr = program[idx + 1];
                let b_adr = program[idx + 2];
                let result_adr = program[idx + 3];
                let result = program[a_adr] + program[b_adr];
                program[result_adr] = result;
                idx = idx + 4;
            }
            CODE_MULTIPLY => {
                let a_adr = program[idx + 1];
                let b_adr = program[idx + 2];
                let result_adr = program[idx + 3];
                let result = program[a_adr] * program[b_adr];
                program[result_adr] = result;
                idx = idx + 4;
            }

            _ => {
                println!("ups, bad code {}", opcode);
            }
        }
    }
    let result = program[0];
    result
}
