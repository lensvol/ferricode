pub mod instruction {
    use std::fmt::{Display, Formatter};
    use crate::opcode::opcode::OpCode;

    #[derive(Debug)]
    pub enum ParameterMode {
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

    pub struct Instruction {
        pub op_code: OpCode,
        pub parameter_modes: Vec<ParameterMode>,
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
}
