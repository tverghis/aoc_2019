#[derive(Debug, PartialEq)]
pub(crate) struct Instruction {
    opcode: OpCode,
    parameters: Option<Vec<Parameter>>,
}

impl Instruction {
    pub(crate) fn new(opcode: OpCode, parameters: Option<Vec<Parameter>>) -> Self {
        Instruction { opcode, parameters }
    }

    pub(crate) fn length(&self) -> usize {
        match self.opcode {
            OpCode::Add | OpCode::Multiply => 4,
            OpCode::Input | OpCode::Output => 2,
            OpCode::Halt => 1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
}

impl From<u32> for OpCode {
    fn from(n: u32) -> Self {
        match n {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            99 => OpCode::Halt,
            _ => panic!("Unknown OpCode: {}", n),
        }
    }
}

impl OpCode {
    pub(crate) fn num_params(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Multiply => 3,
            OpCode::Input | OpCode::Output => 1,
            OpCode::Halt => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Parameter {
    value: i32,
    mode: ParameterMode,
}

impl Parameter {
    pub(crate) fn new(value: i32, mode: ParameterMode) -> Self {
        Parameter { value, mode }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParameterMode {
    Position,
    Immediate,
}

impl From<u32> for ParameterMode {
    fn from(n: u32) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown ParameterMode: {}", n),
        }
    }
}
