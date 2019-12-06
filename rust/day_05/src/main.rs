mod program;

use crate::program::Program;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    let program = Program::from(input.as_str());
    dbg!(program);
}
