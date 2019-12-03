use std::fs;

enum OpCode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl From<usize> for OpCode {
    fn from(i: usize) -> Self {
        match i {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Halt,
            _ => panic!(format!("Unrecognized OpCode: {}", i)),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot open input file");
    let mut instr_arr: Vec<usize> = input
        .trim()
        .split(',')
        .map(|i| i.parse().expect("Could not parse into i8"))
        .collect();

    instr_arr[1] = 12;
    instr_arr[2] = 2;

    run_instructions(&mut instr_arr);

    println!("Part 1: {}", instr_arr[0]);
}

fn run_instructions(instr_arr: &mut [usize]) {
    for pc in (0..instr_arr.len()).step_by(4) {
        match OpCode::from(instr_arr[pc]) {
            OpCode::Add => {
                instr_arr[instr_arr[pc + 3]] =
                    instr_arr[instr_arr[pc + 1]] + instr_arr[instr_arr[pc + 2]];
            }
            OpCode::Multiply => {
                instr_arr[instr_arr[pc + 3]] =
                    instr_arr[instr_arr[pc + 1]] * instr_arr[instr_arr[pc + 2]];
            }
            OpCode::Halt => {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_instructions() {
        let mut instructions = vec![1, 0, 0, 0, 99];
        run_instructions(&mut instructions);
        assert_eq!(instructions, vec![2, 0, 0, 0, 99]);

        let mut instructions = vec![2, 3, 0, 3, 99];
        run_instructions(&mut instructions);
        assert_eq!(instructions, vec![2, 3, 0, 6, 99]);

        let mut instructions = vec![2, 4, 4, 5, 99, 0];
        run_instructions(&mut instructions);
        assert_eq!(instructions, vec![2, 4, 4, 5, 99, 9801]);

        let mut instructions = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_instructions(&mut instructions);
        assert_eq!(instructions, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
