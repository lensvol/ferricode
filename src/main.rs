use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfZero,
    JumpIfNotZero,
    StoreIfLessThan,
    StoreIfEquals,
    IncrementRelativeBase,
    Halt,
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl Display for ParameterMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterMode::Position => write!(f, "ARG"),
            ParameterMode::Immediate => write!(f, "#"),
            ParameterMode::Relative => write!(f, "@"),
        }
    }
}

struct Instruction {
    op_code: OpCode,
    parameter_modes: Vec<ParameterMode>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op_code)?;
        for mode in &self.parameter_modes {
            write!(f, " {}", mode)?;
        }
        Ok(())
    }
}

impl TryFrom<i32> for Instruction {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let numeric_op_code = value % 100;
        if !(0..=9).contains(&numeric_op_code) && value != 99 {
            return Err("Invalid instruction");
        }

        let value = value as u32;
        let op_code = match value % 100 {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfNotZero,
            6 => OpCode::JumpIfZero,
            7 => OpCode::StoreIfLessThan,
            8 => OpCode::StoreIfEquals,
            9 => OpCode::IncrementRelativeBase,
            99 => OpCode::Halt,
            _ => Err("Invalid instruction")?
        };

        let mut parameter_modes = Vec::new();
        let parameter_count = match op_code {
            OpCode::Add | OpCode::Mul | OpCode::StoreIfLessThan | OpCode::StoreIfEquals => 3,
            OpCode::JumpIfNotZero | OpCode::JumpIfZero => 2,
            OpCode::Input | OpCode::Output | OpCode::IncrementRelativeBase => 1,
            OpCode::Halt => 0,
        };

        for param in 0..parameter_count {
            let mode = match value / 10_u32.pow(param as u32 + 2) % 10 {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
                _ => unreachable!(),
            };

            parameter_modes.push(mode);
        }

        Ok(Instruction {
            op_code,
            parameter_modes,
        })
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Add => write!(f, "ADD"),
            OpCode::Mul => write!(f, "MUL"),
            OpCode::Input => write!(f, "IN"),
            OpCode::Output => write!(f, "OUT"),
            OpCode::JumpIfNotZero => write!(f, "JNZ"),
            OpCode::JumpIfZero => write!(f, "JZ"),
            OpCode::StoreIfLessThan => write!(f, "SLT"),
            OpCode::StoreIfEquals => write!(f, "SEQ"),
            OpCode::IncrementRelativeBase => write!(f, "INCB"),
            OpCode::Halt => write!(f, "HALT"),
        }
    }
}

struct Computer {
    memory: Vec<i32>,
    instruction_pointer: usize,
    relative_base: usize,
    input: VecDeque<i32>,
    output: Vec<i32>,
}

impl Computer {
    fn new(memory: Vec<i32>, input: Vec<i32>) -> Computer {
        Computer {
            memory,
            instruction_pointer: 0,
            relative_base: 0,
            input: VecDeque::from(input),
            output: vec![],
        }
    }

    fn read_addr(&mut self, mode: &ParameterMode) -> usize {
        let addr = match mode {
            ParameterMode::Position | ParameterMode::Immediate => {
                let value = self.memory[self.instruction_pointer];
                self.instruction_pointer += 1;
                value
            },
            ParameterMode::Relative => (self.instruction_pointer + self.relative_base) as i32
        };

        if addr < 0 {
            panic!("Invalid address: {}", addr);
        }

        addr as usize
    }

    fn read_value(&mut self, mode: &ParameterMode) -> i32 {
        return match mode {
            ParameterMode::Immediate => {
                let value = self.memory[self.instruction_pointer];
                self.instruction_pointer += 1;
                value
            }
            _ => {
                let addr = self.read_addr(mode);
                self.memory[addr]
            }
        }
    }

    fn step_forward(&mut self) -> bool {
        let instruction = Instruction::try_from(self.memory[self.instruction_pointer])
            .expect("Failed to decode instruction");

        print!("[IP: {} RB: {}] ", self.instruction_pointer, self.relative_base);

        if let OpCode::Halt = instruction.op_code {
            println!("HALT");
            return true;
        }
        self.instruction_pointer += 1;

        match instruction.op_code {
            OpCode::Add => {
                let arg1 = self.read_value(&instruction.parameter_modes[0]);
                let arg2 = self.read_value(&instruction.parameter_modes[1]);
                let addr = self.read_addr(&instruction.parameter_modes[2]);

                println!("ADD {} {} {}", arg1, arg2, addr);
                self.memory[addr] = arg1 + arg2;
            }
            OpCode::Mul => {
                let arg1 = self.read_value(&instruction.parameter_modes[0]);
                let arg2 = self.read_value(&instruction.parameter_modes[1]);
                let addr = self.read_addr(&instruction.parameter_modes[2]);

                println!("ADD {} {} {}", arg1, arg2, addr);
                self.memory[addr] = arg1 * arg2;
            }
            OpCode::Input => {
                let addr = self.read_addr(&instruction.parameter_modes[0]);
                let input = self.input.pop_front().expect("Input exhausted!");

                println!("IN {} {}", input, addr);
                self.memory[addr] = input;
            }
            OpCode::Output => {
                let value = self.read_value(&instruction.parameter_modes[0]);

                println!("OUT {}", value);
                self.output.push(value);
            }
            OpCode::JumpIfZero => {
                let test = self.read_value(&instruction.parameter_modes[0]);
                let addr = self.read_value(&instruction.parameter_modes[1]) as usize;

                println!("JZ {} {}", test, addr);
                if test == 0 {
                    self.instruction_pointer = addr
                }
            }
            OpCode::JumpIfNotZero => {
                let test = self.read_value(&instruction.parameter_modes[0]);
                let addr = self.read_value(&instruction.parameter_modes[1]) as usize;

                println!("JZ {} {}", test, addr);
                if test != 0 {
                    self.instruction_pointer = addr
                }
            }
            OpCode::StoreIfLessThan => {
                let arg1 = self.read_value(&instruction.parameter_modes[0]);
                let arg2 = self.read_value(&instruction.parameter_modes[1]);
                let addr = self.read_addr(&instruction.parameter_modes[2]);

                println!("SLT {} {} {}", arg1, arg2, addr);
                self.memory[addr] = if arg1 < arg2 { 1 } else { 0 };
            }
            OpCode::StoreIfEquals => {
                let arg1 = self.read_value(&instruction.parameter_modes[0]);
                let arg2 = self.read_value(&instruction.parameter_modes[1]);
                let addr = self.read_addr(&instruction.parameter_modes[2]);

                println!("SEQ {} {} {}", arg1, arg2, addr);
                self.memory[addr] = if arg1 == arg2 { 1 } else { 0 };
            }
            OpCode::IncrementRelativeBase => {
                let offset = self.read_value(&instruction.parameter_modes[0]);
                println!("INCB {}", offset);

                self.relative_base += offset as usize;
            }
            OpCode::Halt => unreachable!()
        }

        return false;
    }

    fn run(&mut self) {
        println!("[MEMORY] {:?}", self.memory);
        println!("[INPUT] {:?}", self.input);
        loop {
            let should_halt = self.step_forward();
            if should_halt {
                break;
            }
        }
        println!("[OUTPUT] {:?}", self.output);
        println!("");
    }
}

fn main() {
    let program = vec![
        4,3,101,72,14,3,101,1,4,4,5,3,16,99,29,7,0,3,-67,-12,87,-8,3,-6,-8,-67,-23,-10
    ];

    let mut computer = Computer::new(program, vec![4, 3, 2, 1, 0]);
    computer.run();

    println!("Output: {:?}", computer.output);
}

mod tests {
    use crate::Computer;

    fn run_program(program: Vec<i32>, input: Vec<i32>) -> Computer {
        let mut computer = super::Computer::new(program, input);
        computer.run();
        computer
    }

    #[test]
    fn test_day_2_simple_programs() {
        let computer = run_program(vec![1, 0, 0, 0, 99], vec![]);
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);

        let computer = run_program(vec![2, 3, 0, 3, 99], vec![]);
        assert_eq!(computer.memory, vec![2, 3, 0, 6, 99]);

        let computer = run_program(vec![2, 4, 4, 5, 99, 0], vec![]);
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);

        let computer = run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]);
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_day_5_simple_programs() {
        let computer = run_program(vec![3, 0, 4, 0, 99], vec![42]);
        assert_eq!(computer.output, vec![42]);

        let computer = run_program(vec![1002, 4, 3, 4, 33], vec![]);
        assert_eq!(computer.memory, vec![1002, 4, 3, 4, 99]);

        let computer = run_program(vec![1101, 100, -1, 4, 0], vec![]);
        assert_eq!(computer.memory, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_day_9_simple_programs() {
        let computer = run_program(vec![109, 19, 204, -34, 99], vec![]);
        assert_eq!(computer.output, vec![19]);

        let computer = run_program(vec![109, 1, 9, 2, 204, -6, 99], vec![]);
        assert_eq!(computer.output, vec![204]);

        let computer = run_program(vec![109, 1, 109, 9, 204, -6, 99], vec![]);
        assert_eq!(computer.output, vec![204]);

        let computer = run_program(vec![109, 1, 209, -1, 204, -106, 99], vec![]);
        assert_eq!(computer.output, vec![204]);

        let computer = run_program(vec![109, 1, 3, 3, 204, 2, 99], vec![42]);
        assert_eq!(computer.output, vec![42]);

        let computer = run_program(vec![109, 1, 203, 2, 204, 2, 99], vec![42]);
        assert_eq!(computer.output, vec![42]);
    }

    #[test]
    fn test_quine() {
        let computer = run_program(vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], vec![]);
        assert_eq!(computer.output, vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
    }
}