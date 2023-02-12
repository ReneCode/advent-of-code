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

#[derive(Debug)]
enum StepResult {
    Ok,
    Output(i32),
    Stop,
}

enum Mode {
    Position = 0,
    Immediate = 1,
}

type Program = Vec<i32>;

struct Amplifier {
    programm: Program,
    idx: usize,
    inputs: Vec<i32>,
}

impl Amplifier {
    fn new(line: &str, init_phase_setting: i32) -> Self {
        let mut inputs = Vec::new();
        inputs.push(init_phase_setting);
        Amplifier {
            programm: create_program(line),
            idx: 0,
            inputs: inputs,
        }
    }

    fn run(&mut self) -> StepResult {
        loop {
            let result = step_program(&mut self.programm, &mut self.inputs, &mut self.idx);
            match result {
                StepResult::Stop => break result,
                StepResult::Output(_) => break result,
                StepResult::Ok => {}
            }
        }
    }
}

fn main() {
    println!("Hello, day07!");

    if let Some(input) = util::io::get_lines("./07.data") {
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

fn part_2(line: &str) {
    let mut max_value = 0;
    for combination in (5..=9).permutations(5) {
        let value = run_amplifiers_loop(line, &combination);
        max_value = max_value.max(value);
    }
    println!("part-2 max output:{}", max_value);
}

fn run_sequence(line: &str, phase_settings: &Vec<i32>) -> i32 {
    let mut value = 0;
    for phase_setting in phase_settings {
        let mut program = create_program(line);
        let mut inputs = vec![*phase_setting, value];
        value = run_programm(&mut program, &mut inputs);
    }
    value
}

fn run_amplifiers_loop(line: &str, phase_settings: &Vec<i32>) -> i32 {
    // let mut programms: Vec<Program> = phase_settings
    //     .iter()
    //     .map(|_| create_program(line))
    //     .collect();

    let mut amp_a = Amplifier::new(line, phase_settings[0]);
    let mut amp_b = Amplifier::new(line, phase_settings[1]);
    let mut amp_c = Amplifier::new(line, phase_settings[2]);
    let mut amp_d = Amplifier::new(line, phase_settings[3]);
    let mut amp_e = Amplifier::new(line, phase_settings[4]);

    amp_a.inputs.push(0);
    let mut saved_output_e: i32 = -1;
    let result = loop {
        if let StepResult::Output(output_a) = amp_a.run() {
            amp_b.inputs.push(output_a);
            if let StepResult::Output(output_b) = amp_b.run() {
                amp_c.inputs.push(output_b);
                if let StepResult::Output(output_c) = amp_c.run() {
                    amp_d.inputs.push(output_c);
                    if let StepResult::Output(output_d) = amp_d.run() {
                        amp_e.inputs.push(output_d);
                        if let StepResult::Output(output_e) = amp_e.run() {
                            saved_output_e = output_e;
                            amp_a.inputs.push(output_e);
                        } else {
                            break saved_output_e;
                        }
                    } else {
                        break saved_output_e;
                    }
                } else {
                    break saved_output_e;
                }
            } else {
                break saved_output_e;
            }
        } else {
            break saved_output_e;
        }
    };

    // println!("{:?}", result);
    result
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

#[test]
fn test_4() {
    let line =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(139629729, run_amplifiers_loop(line, &vec![9, 8, 7, 6, 5]))
}

#[test]
fn test_5() {
    let line ="3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    assert_eq!(18216, run_amplifiers_loop(line, &vec![9, 7, 8, 5, 6]))
}

#[test]
fn test_remove() {
    let mut v = vec![1, 2, 3, 4];
    assert_eq!(1, v.remove(0));
    assert_eq!(2, v.remove(0));
    assert_eq!(3, v.remove(0));
    assert_eq!(vec![4], v);
    assert_eq!(4, v.remove(0));
    assert_eq!(0, v.len());
}

fn run_programm(program: &mut Program, inputs: &mut Vec<i32>) -> i32 {
    let mut idx: usize = 0;
    let mut output: i32 = 0;
    loop {
        let step_result = step_program(program, inputs, &mut idx);
        match step_result {
            StepResult::Output(output_value) => output = output_value,
            StepResult::Stop => break,
            StepResult::Ok => {}
        }
    }
    output
}

fn step_program(program: &mut Program, inputs: &mut Vec<i32>, idx: &mut usize) -> StepResult {
    let opcode = program[*idx];
    match opcode {
        CODE_STOP => StepResult::Stop,
        CODE_SAVE_INPUT => {
            let adr = program[*idx + 1] as usize;
            let input_value = inputs.remove(0);
            program[adr] = input_value;
            *idx += 2;
            StepResult::Ok
        }
        CODE_OUTPUT => {
            // todo
            let adr = program[*idx + 1] as usize;
            let output = program[adr];
            *idx += 2;
            StepResult::Output(output)
        }
        _ => run_instruction(opcode, program, idx),
    }
}

fn parse_char_to_int(c: char) -> u32 {
    match c.to_digit(10) {
        Some(n) => n,
        _ => 0,
    }
}

fn run_instruction(instruction: i32, program: &mut Program, idx: &mut usize) -> StepResult {
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
    let mut result = StepResult::Ok;
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
            result = output_with_mode(program, idx, mode_first);
        }

        CODE_JUMP_IF_TRUE => {
            *idx = jump_if(program, *idx, (mode_first, mode_second), |v| v != 0);
        }

        CODE_JUMP_IF_FALSE => {
            *idx = jump_if(program, *idx, (mode_first, mode_second), |v| v == 0);
        }
        CODE_CMP_IF_LT => {
            *idx = cmp_if(
                program,
                *idx,
                (mode_first, mode_second, mode_third),
                |a, b| a < b,
            )
        }
        CODE_CMP_IF_EQ => {
            *idx = cmp_if(
                program,
                *idx,
                (mode_first, mode_second, mode_third),
                |a, b| a == b,
            )
        }
        _ => {
            panic!("bad opcode {}", opcode);
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
    idx: &mut usize,
    modes: (Mode, Mode, Mode),
    calc_fn: fn(i32, i32) -> i32,
) -> StepResult {
    let a_adr = get_adr(program, *idx + 1, modes.0);
    let b_adr = get_adr(program, *idx + 2, modes.1);
    let result_adr = get_adr(program, *idx + 3, modes.2);
    let result = calc_fn(program[a_adr], program[b_adr]);
    program[result_adr] = result;
    *idx += 4;
    StepResult::Ok
}

fn output_with_mode(program: &[i32], idx: &mut usize, mode_parameter: Mode) -> StepResult {
    let adr = get_adr(program, *idx + 1, mode_parameter);
    let output = program[adr];
    *idx += 2;
    StepResult::Output(output)
}
