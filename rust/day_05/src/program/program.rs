use crate::program::{OpCode, Parameter, ParameterMode};
use std::convert::TryInto;
use std::io;

#[derive(Debug)]
pub(crate) struct Program {
    inputs: Vec<i32>,
}

impl Program {
    pub(crate) fn run(&mut self) {
        let mut i = 0;

        loop {
            let mut instruction = self.inputs[i];
            let opcode = OpCode::from(instruction % 100);
            instruction /= 100;

            if opcode == OpCode::Halt {
                break;
            }

            let mut parameters = Vec::new();
            for p in 0..opcode.num_params() {
                let p_u32 = p.try_into().unwrap();
                let param_mode =
                    ParameterMode::from((instruction % 10_i32.pow(p_u32 + 1)) / 10_i32.pow(p_u32));
                let param_value = self.inputs[i + p + 1];
                parameters.push(Parameter::new(param_value, param_mode));
            }

            i += opcode.instr_len();
            self.do_instruction((opcode, parameters))
        }
    }

    fn do_instruction(&mut self, (opcode, params): (OpCode, Vec<Parameter>)) {
        match opcode {
            OpCode::Halt => unreachable!(),
            OpCode::Input => self.do_input(&params),
            OpCode::Output => self.do_output(&params),
            OpCode::Add => self.do_add(&params),
            OpCode::Multiply => self.do_multiply(&params),
        }
    }

    fn do_input(&mut self, parameters: &[Parameter]) {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Unable to read input");

        let input = input.trim().parse().unwrap();
        self.inputs[parameters[0].value as usize] = input;
    }

    fn do_output(&mut self, parameters: &[Parameter]) {
        println!("{}", self.inputs[parameters[0].value as usize]);
    }

    fn do_add(&mut self, parameters: &[Parameter]) {
        let x = self.get_parameter_value(&parameters[0]);
        let y = self.get_parameter_value(&parameters[1]);
        let output_idx = parameters[2].value;

        self.inputs[output_idx as usize] = x + y;
    }

    fn do_multiply(&mut self, parameters: &[Parameter]) {
        let x = self.get_parameter_value(&parameters[0]);
        let y = self.get_parameter_value(&parameters[1]);
        let output_idx = parameters[2].value;

        self.inputs[output_idx as usize] = x * y;
    }

    fn get_parameter_value(&self, parameter: &Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Position => self.inputs[parameter.value as usize],
        }
    }
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Program {
            inputs: input
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect(),
        }
    }
}
