use crate::program::{Instruction, OpCode, Parameter, ParameterMode};
use std::cmp::Ordering;
use std::convert::TryInto;
use std::io;

#[derive(Debug)]
pub(crate) struct Program {
    inputs: Vec<i32>,
    instr_ptr: usize,
}

impl Program {
    pub(crate) fn run(&mut self) {
        loop {
            let mut instruction = self.inputs[self.instr_ptr];
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
                let param_value = self.inputs[self.instr_ptr + p + 1];
                parameters.push(Parameter::new(param_value, param_mode));
            }

            self.do_instruction(Instruction::new(opcode, parameters));
        }
    }

    fn do_instruction(&mut self, instr: Instruction) {
        match instr.opcode {
            OpCode::Halt => unreachable!(),
            OpCode::Input => self.do_input(&instr),
            OpCode::Output => self.do_output(&instr),
            OpCode::Add => self.do_add(&instr),
            OpCode::Multiply => self.do_multiply(&instr),
            OpCode::JumpIfTrue => self.do_jump_if_true(&instr),
            OpCode::JumpIfFalse => self.do_jump_if_false(&instr),
            OpCode::LessThan => self.do_less_than(&instr),
            OpCode::Equals => self.do_equals(&instr),
        };
    }

    fn do_input(&mut self, instr: &Instruction) {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Unable to read input");

        let input = input.trim().parse().unwrap();
        self.inputs[instr.parameters[0].value as usize] = input;

        self.instr_ptr += instr.opcode.instr_len();
    }

    fn do_output(&mut self, instr: &Instruction) {
        println!("{}", self.get_parameter_value(instr.parameters[0]));

        self.instr_ptr += instr.opcode.instr_len();
    }

    fn do_add(&mut self, instr: &Instruction) {
        let x = self.get_parameter_value(instr.parameters[0]);
        let y = self.get_parameter_value(instr.parameters[1]);
        let output_idx = instr.parameters[2].value as usize;

        self.inputs[output_idx] = x + y;

        self.instr_ptr += instr.opcode.instr_len();
    }

    fn do_multiply(&mut self, instr: &Instruction) {
        let x = self.get_parameter_value(instr.parameters[0]);
        let y = self.get_parameter_value(instr.parameters[1]);
        let output_idx = instr.parameters[2].value as usize;

        self.inputs[output_idx] = x * y;

        self.instr_ptr += instr.opcode.instr_len();
    }

    fn do_jump_if_true(&mut self, instr: &Instruction) {
        self.do_jump(instr, true);
    }

    fn do_jump_if_false(&mut self, instr: &Instruction) {
        self.do_jump(instr, false);
    }

    fn do_less_than(&mut self, instr: &Instruction) {
        self.do_comparison(instr, Ordering::Less);
    }

    fn do_equals(&mut self, instr: &Instruction) {
        self.do_comparison(instr, Ordering::Equal);
    }

    fn get_parameter_value(&self, parameter: Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Position => self.inputs[parameter.value as usize],
        }
    }

    fn do_jump(&mut self, instr: &Instruction, jump_cond: bool) {
        let x = self.get_parameter_value(instr.parameters[0]);
        let y = self.get_parameter_value(instr.parameters[1]);

        if jump_cond == (x != 0) {
            self.instr_ptr = y as usize;
        } else {
            self.instr_ptr += instr.opcode.instr_len();
        }
    }

    fn do_comparison(&mut self, instr: &Instruction, ordering: Ordering) {
        let x = self.get_parameter_value(instr.parameters[0]);
        let y = self.get_parameter_value(instr.parameters[1]);
        let output_idx = instr.parameters[2].value as usize;

        self.inputs[output_idx] = if ordering == x.cmp(&y) { 1 } else { 0 };

        self.instr_ptr += instr.opcode.instr_len();
    }
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Program {
            inputs: input
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect(),
            instr_ptr: 0,
        }
    }
}
