use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum ProgramStatus {
    Success,
    WaitingForInput,
    Finished,
}

pub fn exec_prog(prog: &[i32], input: Vec<i32>) -> VecDeque<i32> {
    let mut prog   = prog.to_vec();
    let mut ip     = 0;
    let mut output = VecDeque::new();
    let mut inputs = VecDeque::new();
    input.iter().for_each(|e| inputs.push_back(*e));
    while exec_instr(&mut prog, &mut ip, &mut inputs, &mut output) != ProgramStatus::Finished {}
    return output;
}

pub fn exec_instr(
    prog:   &mut [i32],
    ip:     &mut usize,
    input:  &mut VecDeque<i32>,
    output: &mut VecDeque<i32>,
) -> ProgramStatus {
    let op_code =  prog[*ip] %    100;
    let mode1   = (prog[*ip] %   1000) /   100 == 1;
    let mode2   = (prog[*ip] %  10000) /  1000 == 1;
    let mode3   = (prog[*ip] % 100000) / 10000 == 1;

    let get_pos = |mode, pos| {
        if mode {
            pos
        } else {
            prog[pos] as usize
        }
    };

    match op_code {
        1 => {
            // Add
            let val1   = prog[get_pos(mode1, *ip + 1)];
            let val2   = prog[get_pos(mode2, *ip + 2)];
            let pos    = get_pos(mode3, *ip + 3);
            prog[pos]  = val1 + val2;
            *ip        += 4;
        }
        2 => {
            // Multiply
            let val1   = prog[get_pos(mode1, *ip + 1)];
            let val2   = prog[get_pos(mode2, *ip + 2)];
            let pos    = get_pos(mode3, *ip + 3);
            prog[pos]  = val1 * val2;
            *ip        += 4;
        }
        3 => {
            // Input
            if let Some(val) = input.pop_front() {
                let pos    = prog[*ip + 1] as usize;
                prog[pos]  = val;
                *ip       += 2;
            } else {
                return ProgramStatus::WaitingForInput;
            }
        }
        4 => {
            // Output
            let val = prog[get_pos(mode1, *ip + 1)];
            output.push_back(val);
            *ip += 2;
        }
        5 => {
            // Jump-If-True
            let val1 = prog[get_pos(mode1, *ip + 1)];
            let val2 = prog[get_pos(mode2, *ip + 2)];
            if val1 != 0 {
                *ip = val2 as usize;
            } else {
                *ip += 3;
            }
        }
        6 => {
            // Jump-If-False
            let val1 = prog[get_pos(mode1, *ip + 1)];
            let val2 = prog[get_pos(mode2, *ip + 2)];
            if val1 == 0 {
                *ip = val2 as usize;
            } else {
                *ip += 3;
            }
        }
        7 => {
            // Less Than
            let val1 = prog[get_pos(mode1, *ip + 1)];
            let val2 = prog[get_pos(mode2, *ip + 2)];
            let pos  = get_pos(mode3, *ip + 3);
            if val1 < val2 {
                prog[pos] = 1;
            } else {
                prog[pos] = 0;
            }
            *ip += 4;
        }
        8 => {
            // Equals
            let val1 = prog[get_pos(mode1, *ip + 1)];
            let val2 = prog[get_pos(mode2, *ip + 2)];
            let pos  = get_pos(mode3, *ip + 3);
            if val1 == val2 {
                prog[pos] = 1;
            } else {
                prog[pos] = 0;
            }
            *ip += 4;
        }
        99 => return ProgramStatus::Finished,
        _  => panic!("Invalid op code!"),
    }
    return ProgramStatus::Success;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_program1() {
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(exec_prog(&prog, vec![8]), vec![1]);
        assert_eq!(exec_prog(&prog, vec![7]), vec![0]);
    }

    #[test]
    fn example_program2() {
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(exec_prog(&prog, vec![7]), vec![1]);
        assert_eq!(exec_prog(&prog, vec![8]), vec![0]);
    }

    #[test]
    fn example_program3() {
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(exec_prog(&prog, vec![8]), vec![1]);
        assert_eq!(exec_prog(&prog, vec![7]), vec![0]);
        assert_eq!(exec_prog(&prog, vec![9]), vec![0]);
    }

    #[test]
    fn example_program4() {
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(exec_prog(&prog, vec![7]), vec![1]);
        assert_eq!(exec_prog(&prog, vec![8]), vec![0]);
        assert_eq!(exec_prog(&prog, vec![9]), vec![0]);
    }

    #[test]
    fn example_program5() {
        // Here are some jump tests that take an input, then output 0 if the
        // input was zero or 1 if the input was non-zero:
        let prog1 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(exec_prog(&prog1, vec![0]),   vec![0]);
        assert_eq!(exec_prog(&prog1, vec![100]), vec![1]);
        assert_eq!(exec_prog(&prog1, vec![-1]),  vec![1]);

        let prog2 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(exec_prog(&prog2, vec![0]),   vec![0]);
        assert_eq!(exec_prog(&prog2, vec![100]), vec![1]);
        assert_eq!(exec_prog(&prog2, vec![-1]),  vec![1]);
    }

    #[test]
    fn example_program6() {
        // The program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8,
        // output 1000 if the input value is equal to 8, or output
        // 1001 if the input value is greater than 8.
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(exec_prog(&prog, vec![7]), vec![999]);
        assert_eq!(exec_prog(&prog, vec![8]), vec![1000]);
        assert_eq!(exec_prog(&prog, vec![9]), vec![1001]);
    }
}
