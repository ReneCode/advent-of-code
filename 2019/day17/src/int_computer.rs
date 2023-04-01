const CODE_STOP: i64 = 99;
const CODE_ADD: i64 = 1;
const CODE_MULTIPLY: i64 = 2;
const CODE_SAVE_INPUT: i64 = 3;
const CODE_OUTPUT: i64 = 4;
const CODE_JUMP_IF_TRUE: i64 = 5;
const CODE_JUMP_IF_FALSE: i64 = 6;
const CODE_CMP_IF_LT: i64 = 7;
const CODE_CMP_IF_EQ: i64 = 8;
const CODE_RELATIVE_BASE: i64 = 9;

const MODE_POSITION: i64 = 0;
const MODE_IMMEDIATE: i64 = 1;
const MODE_RELATIVE: i64 = 2;

#[derive(Debug)]
enum StepResult {
    Ok,
    Output(i64),
    Stop,
}

type Program = Vec<i64>;

pub trait TakeInputOutput {
    fn read_input(&mut self) -> i64;
    fn take_output(&mut self, value: i64);
}

enum Parameter {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl Parameter {
    pub fn new(mode: i64, value: i64) -> Self {
        match mode {
            MODE_IMMEDIATE => Parameter::Immediate(value),
            MODE_POSITION => Parameter::Position(value),
            MODE_RELATIVE => Parameter::Relative(value),
            _ => panic!("bad mode: {mode}"),
        }
    }
}

enum Instruction {
    Stop,
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    SaveInput(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    CompareIfLessThan(Parameter, Parameter, Parameter),
    CompareIfEqual(Parameter, Parameter, Parameter),
    AdjustRelativeBase(Parameter),
}

pub struct IntComputer {
    programm: Program,
    address: usize,
    relative_base: usize,
}

impl Instruction {
    pub fn execute(
        &self,
        computer: &mut IntComputer,
        device: &mut impl TakeInputOutput,
    ) -> StepResult {
        let result = match self {
            Self::Stop => StepResult::Stop,
            Self::Add(a, b, c) => {
                let val_a = computer.read_value(a);
                let val_b = computer.read_value(b);
                computer.write_value(c, val_a + val_b);
                StepResult::Ok
            }
            Self::Multiply(a, b, c) => {
                let val_a = computer.read_value(a);
                let val_b = computer.read_value(b);
                computer.write_value(c, val_a * val_b);
                StepResult::Ok
            }
            Self::SaveInput(a) => {
                let val = device.read_input();
                computer.write_value(a, val);
                StepResult::Ok
            }
            Self::Output(a) => {
                let val = computer.read_value(a);
                StepResult::Output(val)
            }
            Self::JumpIfTrue(a, b) => {
                let val_a = computer.read_value(a);
                let val_b = computer.read_value(b);
                if val_a != 0 {
                    computer.address = val_b as usize;
                }
                StepResult::Ok
            }
            Self::JumpIfFalse(a, b) => {
                let val_a = computer.read_value(a);
                let val_b = computer.read_value(b);
                if val_a == 0 {
                    computer.address = val_b as usize;
                }
                StepResult::Ok
            }
            Self::CompareIfLessThan(a, b, c) => {
                let val_a = computer.read_value(a);
                let val_b = computer.read_value(b);
                let val = if val_a < val_b { 1 } else { 0 };
                computer.write_value(c, val);
                StepResult::Ok
            }
            Self::CompareIfEqual(a, b, c) => {
                let val_a = computer.read_value(a);
                let val_b = computer.read_value(b);
                let val = if val_a == val_b { 1 } else { 0 };
                computer.write_value(c, val);
                StepResult::Ok
            }
            Self::AdjustRelativeBase(a) => {
                let val_a = computer.read_value(a);
                computer.relative_base = ((computer.relative_base as i64) + val_a) as usize;
                StepResult::Ok
            } // _ => panic!("ups not handled instruction"),
        };
        result
    }
}

struct DemoDevice {
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl TakeInputOutput for DemoDevice {
    fn read_input(&mut self) -> i64 {
        let val = self.inputs.remove(0);
        val
    }

    fn take_output(&mut self, value: i64) {
        self.outputs.push(value);
    }
}

impl DemoDevice {
    fn new() -> Self {
        DemoDevice {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }
}

impl IntComputer {
    pub fn new(line: &str) -> Self {
        IntComputer {
            programm: create_program(line),
            address: 0,
            relative_base: 0,
        }
    }

    pub fn run(&mut self, device: &mut impl TakeInputOutput) {
        loop {
            let result = self.step(device);
            match result {
                StepResult::Stop => break,
                StepResult::Output(val) => device.take_output(val),
                StepResult::Ok => {}
            }
        }
    }

    fn read(&mut self) -> i64 {
        let code = self.programm[self.address];
        self.address += 1;
        code
    }

    fn step(&mut self, device: &mut impl TakeInputOutput) -> StepResult {
        let code = self.read();
        let opcode = code % 100;
        let mode_a = (code / 100) % 10;
        let mode_b = (code / 1000) % 10;
        let mode_c = (code / 10000) % 10;
        let instruction = match opcode {
            CODE_STOP => Instruction::Stop,
            CODE_ADD => Instruction::Add(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_MULTIPLY => Instruction::Multiply(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_SAVE_INPUT => Instruction::SaveInput(Parameter::new(mode_a, self.read())),
            CODE_OUTPUT => Instruction::Output(Parameter::new(mode_a, self.read())),

            CODE_JUMP_IF_TRUE => Instruction::JumpIfTrue(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
            ),
            CODE_JUMP_IF_FALSE => Instruction::JumpIfFalse(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
            ),
            CODE_CMP_IF_EQ => Instruction::CompareIfEqual(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_CMP_IF_LT => Instruction::CompareIfLessThan(
                Parameter::new(mode_a, self.read()),
                Parameter::new(mode_b, self.read()),
                Parameter::new(mode_c, self.read()),
            ),
            CODE_RELATIVE_BASE => {
                Instruction::AdjustRelativeBase(Parameter::new(mode_a, self.read()))
            }
            _ => panic!("bad opcode {opcode}"),
        };

        instruction.execute(self, device)
    }

    fn read_value(&mut self, parameter: &Parameter) -> i64 {
        match parameter {
            Parameter::Immediate(address) => {
                return *address;
            }
            Parameter::Position(address) => {
                let adr = *address as usize;
                self.resize_if_necessary(adr);
                return self.programm[adr];
            }
            Parameter::Relative(address) => {
                let adr = (*address as i64 + self.relative_base as i64) as usize;
                self.resize_if_necessary(adr);
                return self.programm[adr];
            }
        };
    }

    fn write_value(&mut self, parameter: &Parameter, value: i64) {
        match parameter {
            Parameter::Immediate(_address) => {
                panic!("canot write in immediate mode")
            }
            Parameter::Position(address) => {
                let adr = *address as usize;
                self.resize_if_necessary(adr);
                self.programm[adr] = value
            }
            Parameter::Relative(address) => {
                let adr = (*address as i64 + self.relative_base as i64) as usize;
                self.resize_if_necessary(adr);
                self.programm[adr] = value
            }
        }
    }

    fn resize_if_necessary(&mut self, address: usize) {
        if address >= self.programm.len() {
            self.programm.resize(address + 1, 0);
        }
    }
}

fn create_program(line: &str) -> Program {
    let program: Program = line
        .split(",")
        .map(|s| {
            if let Ok(val) = s.parse::<i64>() {
                val
            } else {
                0
            }
        })
        .collect();
    program
}
