pub(crate) struct Instruction {
    opcode: OpCode,
    parameters: Vec<Parameter>,
}

impl Instruction {
    pub(crate) fn new(opcode: OpCode, parameters: Vec<Parameter>) -> Self {
        Instruction { opcode, parameters }
    }
}

pub(crate) enum OpCode {
    Add(u32),
    Multiply(u32),
    Input(u32),
    Output(u32),
    Halt(u32),
}

impl From<u32> for OpCode {
    fn from(n: u32) -> Self {
        match n {
            1 => OpCode::Add(4),
            2 => OpCode::Multiply(4),
            3 => OpCode::Input(2),
            4 => OpCode::Output(2),
            99 => OpCode::Halt(1),
            _ => panic!("Unknown OpCode: {}", n),
        }
    }
}

pub(crate) struct Parameter {
    value: i32,
    mode: ParameterMode,
}

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
