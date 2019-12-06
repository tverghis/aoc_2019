use crate::program::Instruction;

#[derive(Debug)]
pub(crate) struct Program {
    inputs: Vec<String>,
    program_counter: u32,
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Program {
            inputs: input.split(',').map(|s| s.trim().to_string()).collect(),
            program_counter: 0
        }
    }
}

impl Iterator for Program {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}
