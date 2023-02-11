extern crate util;

use itertools::Itertools;

const CODE_STOP: i32 = 99;
const CODE_ADD: i32 = 1;
const CODE_MULTIPLY: i32 = 2;
const CODE_SAVE_INPUT: i32 = 3;
const CODE_OUTPUT: i32 = 4;
const CODE_JUMP_IF_TRUE: i32 = 5;
const CODE_JUMP_IF_FALSE: i32 = 6;
const CODE_CMP_IF_LT: i32 = 7;
const CODE_CMP_IF_EQ: i32 = 8;

enum Mode {
    Position = 0,
    Immediate = 1,
}

type Program = Vec<i32>;

fn main() {
    println!("Hello, day07!");

    if let Some(input) = util::io::get_lines("./07.data") {
        if let Some(line) = input.get(0) {
            part_1(line.as_str());
            // part_2(line.as_str());
        }
    }
}

fn create_program(line: &str) -> Program {
    let program: Program = line
        .split(",")
        .map(|s| {
            if let Ok(val) = s.parse::<i32>() {
                val
            } else {
                0
            }
        })
        .collect();
    program
}

fn part_1(line: &str) {
    let combinations = (0..5).permutations(5);
    let mut max_value = 0;
    for combination in combinations {
        let value = run_sequence(line, &combination);
        max_value = max_value.max(value);
        // println!("{:?} / {}", combination, value);
    }
    println!("part-1 max output:{}", max_value);
}

fn run_sequence(line: &str, phase_settings: &Vec<i32>) -> i32 {
    let mut value = 0;
    for phase_setting in phase_settings {
        let mut program = create_program(line);
        value = work_programm(&mut program, vec![*phase_setting, value]);
    }
    value
}

#[test]
fn test_a() {
    let line = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    assert_eq!(43210, run_sequence(line, &vec![4, 3, 2, 1, 0]));
}

#[test]
fn test_2() {
    let line = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    assert_eq!(54321, run_sequence(line, &vec![0, 1, 2, 3, 4]));
}

#[test]
fn test_3() {
    let line = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    assert_eq!(65210, run_sequence(line, &vec![1, 0, 4, 3, 2]));
}

fn work_programm(program: &mut Program, input: Vec<i32>) -> i32 {
    let mut idx: usize = 0;
    let mut output: i32 = 0;
    let mut input_iter = input.iter();
    loop {
        let opcode = program[idx];
        match opcode {
            CODE_STOP => break,
            CODE_SAVE_INPUT => {
                // todo
                let adr = program[idx + 1] as usize;
                let input_value = input_iter.next().unwrap();
                program[adr] = *input_value;
                idx += 2;
            }
            CODE_OUTPUT => {
                // todo
                let adr = program[idx + 1] as usize;
                output = program[adr];
                idx += 2;
            }
            _ => {
                idx = run_instruction(opcode, program, idx, &mut output);
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

fn run_instruction(instruction: i32, program: &mut Program, idx: usize, output: &mut i32) -> usize {
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

        CODE_JUMP_IF_TRUE => {
            result = jump_if(program, idx, (mode_first, mode_second), |v| v != 0);
        }

        CODE_JUMP_IF_FALSE => {
            result = jump_if(program, idx, (mode_first, mode_second), |v| v == 0);
        }
        CODE_CMP_IF_LT => {
            result = cmp_if(
                program,
                idx,
                (mode_first, mode_second, mode_third),
                |a, b| a < b,
            )
        }
        CODE_CMP_IF_EQ => {
            result = cmp_if(
                program,
                idx,
                (mode_first, mode_second, mode_third),
                |a, b| a == b,
            )
        }
        _ => {
            panic!("bad opcode {}", opcode);
            result = 0
        }
    }

    result
}

fn cmp_if(
    program: &mut Program,
    idx: usize,
    modes: (Mode, Mode, Mode),
    cmp_fn: fn(i32, i32) -> bool,
) -> usize {
    let a_adr = get_adr(program, idx + 1, modes.0);
    let b_adr = get_adr(program, idx + 2, modes.1);
    let result_adr = get_adr(program, idx + 3, modes.2);
    let val_a = program[a_adr];
    let val_b = program[b_adr];
    if cmp_fn(val_a, val_b) {
        program[result_adr] = 1;
    } else {
        program[result_adr] = 0;
    }
    idx + 4
}

fn jump_if(program: &[i32], idx: usize, modes: (Mode, Mode), check_fn: fn(i32) -> bool) -> usize {
    let a_adr = get_adr(program, idx + 1, modes.0);
    let b_adr = get_adr(program, idx + 2, modes.1);
    let val_a = program[a_adr];
    let val_b = program[b_adr] as usize;
    if check_fn(val_a) {
        val_b
    } else {
        idx + 3
    }
}

fn get_adr(program: &[i32], idx: usize, mode: Mode) -> usize {
    match mode {
        Mode::Position => program[idx] as usize,
        Mode::Immediate => idx,
    }
}
fn calc_with_mode(
    program: &mut Program,
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
