use crate::program::{Instruction, OpCode, Parameter, ParameterMode};

#[derive(Debug)]
pub(crate) struct Program {
    inputs: Vec<String>,
    program_counter: usize,
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Program {
            inputs: input.split(',').map(|s| s.trim().to_string()).collect(),
            program_counter: 0,
        }
    }
}

impl Iterator for Program {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut current_instruction) = self
            .inputs
            .get(self.program_counter)
            .map(|i| i.parse::<u32>().unwrap())
        {
            // Parse the instruction to get the OpCode and the parameters
            let opcode = OpCode::from(current_instruction % 100);
            current_instruction /= 100;

            if opcode == OpCode::Halt {
                return Some(Instruction::new(opcode, None));
            }

            let mut parameters = Vec::new();

            for i in 1..=opcode.num_params() {
                let param_value = self.inputs[self.program_counter + i]
                    .parse::<i32>()
                    .unwrap();
                let param_mode = ParameterMode::from(current_instruction % 10);

                let parameter = Parameter::new(param_value, param_mode);
                parameters.push(parameter);

                current_instruction /= 10;
            }

            let next_instruction = Instruction::new(opcode, Some(parameters));

            self.program_counter += next_instruction.length();

            Some(next_instruction)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_instruction() {
        let mut program = Program::from("1002,4,3,5,99");
        assert_eq!(program.next(), None);
    }
}
