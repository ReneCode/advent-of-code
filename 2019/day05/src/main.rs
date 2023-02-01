extern crate util;

const CODE_STOP: i32 = 99;
const CODE_ADD: i32 = 1;
const CODE_MULTIPLY: i32 = 2;
const CODE_SAVE_INPUT: i32 = 3;
const CODE_OUTPUT: i32 = 4;

enum Mode {
    Position = 0,
    Immediate = 1,
}

fn main() {
    println!("Hello, day05!");

    if let Some(input) = util::io::get_lines("./05.data") {
        if let Some(line) = input.get(0) {
            part_1(line.as_str());
        }
    }
}

fn part_1(line: &str) {
    let result = work_programm(line, 1);
    println!("part-1 output: {}", result)
}

#[test]
fn simple_input_output() {
    let programm = "3,0,4,0,99";
    assert_eq!(work_programm(programm, 44), 44);
}

fn work_programm(line: &str, input: i32) -> i32 {
    let mut program: Vec<i32> = line
        .split(",")
        .map(|s| {
            if let Ok(val) = s.parse::<i32>() {
                val
            } else {
                0
            }
        })
        .collect();

    let mut idx: usize = 0;
    let mut output: i32 = 0;
    loop {
        let opcode = program[idx];
        match opcode {
            CODE_STOP => break,
            CODE_ADD => {
                idx = calc_with_mode(
                    &mut program,
                    idx,
                    (Mode::Position, Mode::Position, Mode::Position),
                    |a, b| a + b,
                );
            }
            CODE_MULTIPLY => {
                idx = calc_with_mode(
                    &mut program,
                    idx,
                    (Mode::Position, Mode::Position, Mode::Position),
                    |a, b| a * b,
                );
            }
            CODE_SAVE_INPUT => {
                // todo
                let adr = program[idx + 1] as usize;
                program[adr] = input;
                idx += 2;
            }
            CODE_OUTPUT => {
                // todo
                let adr = program[idx + 1] as usize;
                output = program[adr];
                idx += 2;
            }
            _ => {
                idx = run_instruction(opcode, &mut program, idx, &mut output);
            }
        }
    }
    output
}

fn parse_char_to_int(c: char) -> u32 {
    match c.to_digit(10) {
        Some(n) => n,
        _ => 0,
    }
}

fn run_instruction(
    instruction: i32,
    program: &mut Vec<i32>,
    idx: usize,
    output: &mut i32,
) -> usize {
    let reverse_instruction: Vec<i32> = instruction
        .to_string()
        .chars()
        .map(|c| parse_char_to_int(c) as i32)
        .rev()
        .collect();

    let opcode = reverse_instruction[0];

    let mut mode_first = Mode::Position;
    let mut mode_second = Mode::Position;
    let mut mode_third = Mode::Position;
    if reverse_instruction.len() >= 3 && reverse_instruction[2] == 1 {
        mode_first = Mode::Immediate
    }
    if reverse_instruction.len() >= 4 && reverse_instruction[3] == 1 {
        mode_second = Mode::Immediate
    }
    if reverse_instruction.len() >= 5 && reverse_instruction[4] == 1 {
        mode_third = Mode::Immediate
    }
    let result: usize;
    match opcode {
        CODE_ADD => {
            result = calc_with_mode(
                program,
                idx,
                (mode_first, mode_second, mode_third),
                |a, b| a + b,
            );
        }
        CODE_MULTIPLY => {
            result = calc_with_mode(
                program,
                idx,
                (mode_first, mode_second, mode_third),
                |a, b| a * b,
            );
        }
        CODE_OUTPUT => {
            result = output_with_mode(program, idx, mode_first, output);
        }
        _ => {
            panic!("bad opcode {}", opcode);
            result = 0
        }
    }

    result
}

fn get_adr(program: &[i32], idx: usize, mode: Mode) -> usize {
    match mode {
        Mode::Position => program[idx] as usize,
        Mode::Immediate => idx,
    }
}
fn calc_with_mode(
    program: &mut Vec<i32>,
    idx: usize,
    modes: (Mode, Mode, Mode),
    calc_fn: fn(i32, i32) -> i32,
) -> usize {
    let a_adr = get_adr(program, idx + 1, modes.0);
    let b_adr = get_adr(program, idx + 2, modes.1);
    let result_adr = get_adr(program, idx + 3, modes.2);
    let result = calc_fn(program[a_adr], program[b_adr]);
    program[result_adr] = result;
    idx + 4
}

fn output_with_mode(program: &[i32], idx: usize, mode_parameter: Mode, output: &mut i32) -> usize {
    let adr = get_adr(program, idx + 1, mode_parameter);
    *output = program[adr];
    idx + 2
}
