#[derive(Debug)]
pub(crate) struct Instruction {
    pub(crate) opcode: OpCode,
    pub(crate) parameters: Vec<Parameter>,
}

impl Instruction {
    pub(crate) fn new(opcode: OpCode, parameters: Vec<Parameter>) -> Self {
        Instruction { opcode, parameters }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl From<i32> for OpCode {
    fn from(n: i32) -> Self {
        match n {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => panic!("Unknown OpCode: {}", n),
        }
    }
}

impl OpCode {
    pub(crate) fn num_params(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 3,
            OpCode::Input | OpCode::Output => 1,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::Halt => 0,
        }
    }

    pub(crate) fn instr_len(&self) -> usize {
        self.num_params() + 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Parameter {
    pub(crate) value: i32,
    pub(crate) mode: ParameterMode,
}

impl Parameter {
    pub(crate) fn new(value: i32, mode: ParameterMode) -> Self {
        Parameter { value, mode }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(n: i32) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown ParameterMode: {}", n),
        }
    }
}
