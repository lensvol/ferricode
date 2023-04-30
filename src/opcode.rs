pub mod opcode {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum OpCode {
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
}
