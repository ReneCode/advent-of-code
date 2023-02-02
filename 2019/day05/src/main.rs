extern crate util;

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

fn main() {
    println!("Hello, day05!");

    if let Some(input) = util::io::get_lines("./05.data") {
        if let Some(line) = input.get(0) {
            part_1(line.as_str());
            part_2(line.as_str());
        }
    }
}

fn part_1(line: &str) {
    let result = work_programm(line, 1);
    println!("part-1 output: {}", result)
}

fn part_2(line: &str) {
    let result = work_programm(line, 5);
    println!("part-1 output: {}", result)
}


#[test]
fn simple_input_output() {
    let programm = "3,0,4,0,99";
    assert_eq!(work_programm(programm, 44), 44);
}

#[test]
fn test_5_less_than_8() {
    let programm = "3,3,1107,-1,8,3,4,3,99";
    assert_eq!(work_programm(programm, 5), 1);
}

#[test]
fn test_9_not_less_than_8() {
    let programm = "3,3,1107,-1,8,3,4,3,99";
    assert_eq!(work_programm(programm, 9), 0);
}

#[test]
fn test_8_equal_8() {
    let programm = "3,9,8,9,10,9,4,9,99,-1,8";
    assert_eq!(work_programm(programm, 8), 1);
}

#[test]
fn test_5_not_equal_8() {
    let programm = "3,9,8,9,10,9,4,9,99,-1,8";
    assert_eq!(work_programm(programm, 5), 0);
}


#[test]
fn test_large_5_below_8() {
    let programm = 
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(work_programm(programm, 5), 999);
}

#[test]
fn test_large_8_eq_8() {
    let programm = 
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(work_programm(programm, 8), 1000);
}

#[test]
fn test_large_12_greater_8() {
    let programm = 
    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(work_programm(programm, 12), 1001);
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
    program: &mut Vec<i32>,
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
