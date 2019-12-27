use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum ProgramStatus {
    Success,
    WaitingForInput,
    Finished,
}

#[derive(Debug, Clone)]
pub struct IntcodeProg {
    mem:      Vec<i64>,
    ip:       usize,
    rel_base: i64,
}

impl IntcodeProg {
    pub fn new(prog: &[i64]) -> IntcodeProg {
        IntcodeProg {
            mem:      prog.to_vec(),
            ip:       0,
            rel_base: 0,
        }
    }

    pub fn exec_prog(prog: &[i64], input: Vec<i64>) -> VecDeque<i64> {
        let mut prog   = IntcodeProg::new(prog);
        let mut output = VecDeque::new();
        let mut inputs = VecDeque::new();
        input.iter().for_each(|e| inputs.push_back(*e));
        loop {
            match prog.exec_instr(&mut inputs, &mut output) {
                ProgramStatus::Success         => (),
                ProgramStatus::Finished        => break output,
                ProgramStatus::WaitingForInput => panic!("Missing input!"),
            }
        }
    }

    pub fn exec_instr(
        &mut self,
        input:  &mut VecDeque<i64>,
        output: &mut VecDeque<i64>,
    ) -> ProgramStatus {
        let op_code =  self.mem[self.ip] %     100;
        let mode1   = (self.mem[self.ip] %   1_000) /    100;
        let mode2   = (self.mem[self.ip] %  10_000) /  1_000;
        let mode3   = (self.mem[self.ip] % 100_000) / 10_000;
        match op_code {
            1 => {
                // Add
                let pos1        = self.get_pos(mode1, self.ip + 1);
                let pos2        = self.get_pos(mode2, self.ip + 2);
                let pos3        = self.get_pos(mode3, self.ip + 3);
                self.mem[pos3]  = self.mem[pos1] + self.mem[pos2];
                self.ip        += 4;
            }
            2 => {
                // Multiply
                let pos1        = self.get_pos(mode1, self.ip + 1);
                let pos2        = self.get_pos(mode2, self.ip + 2);
                let pos3        = self.get_pos(mode3, self.ip + 3);
                self.mem[pos3]  = self.mem[pos1] * self.mem[pos2];
                self.ip        += 4;
            }
            3 => {
                // Input
                if let Some(val) = input.pop_front() {
                    let pos        = self.get_pos(mode1, self.ip + 1);
                    self.mem[pos]  = val;
                    self.ip       += 2;
                } else {
                    return ProgramStatus::WaitingForInput;
                }
            }
            4 => {
                // Output
                let pos = self.get_pos(mode1, self.ip + 1);
                output.push_back(self.mem[pos]);
                self.ip += 2;
            }
            5 => {
                // Jump-If-True
                let pos1 = self.get_pos(mode1, self.ip + 1);
                let pos2 = self.get_pos(mode2, self.ip + 2);
                if self.mem[pos1] != 0 {
                    self.ip = self.mem[pos2] as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                // Jump-If-False
                let pos1 = self.get_pos(mode1, self.ip + 1);
                let pos2 = self.get_pos(mode2, self.ip + 2);
                if self.mem[pos1] == 0 {
                    self.ip = self.mem[pos2] as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                // Less Than
                let pos1 = self.get_pos(mode1, self.ip + 1);
                let pos2 = self.get_pos(mode2, self.ip + 2);
                let pos3 = self.get_pos(mode3, self.ip + 3);
                if self.mem[pos1] < self.mem[pos2] {
                    self.mem[pos3] = 1;
                } else {
                    self.mem[pos3] = 0;
                }
                self.ip += 4;
            }
            8 => {
                // Equals
                let pos1 = self.get_pos(mode1, self.ip + 1);
                let pos2 = self.get_pos(mode2, self.ip + 2);
                let pos3 = self.get_pos(mode3, self.ip + 3);
                if self.mem[pos1] == self.mem[pos2] {
                    self.mem[pos3] = 1;
                } else {
                    self.mem[pos3] = 0;
                }
                self.ip += 4;
            }
            9 => {
                // Adjust the Relative Base
                let pos        = self.get_pos(mode1, self.ip + 1);
                self.rel_base += self.mem[pos];
                self.ip       += 2;
            }
            99 => return ProgramStatus::Finished,
            _  => panic!("Invalid op code!"),
        }
        ProgramStatus::Success
    }

    fn get_pos(&mut self, mode: i64, pos: usize) -> usize {
        let pos = match mode {
            0 => self.mem[pos] as usize,
            1 => pos,
            2 => (self.rel_base + self.mem[pos]) as usize,
            _ => panic!("Invalid parameter mode!"),
        };
        // Increase size if position is outside of the currently initialized memory.
        if pos >= self.mem.len() {
            for _ in self.mem.len()..=pos {
                self.mem.push(0);
            }
        }
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_program1() {
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![8]), vec![1]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![7]), vec![0]);
    }

    #[test]
    fn example_program2() {
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![7]), vec![1]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![8]), vec![0]);
    }

    #[test]
    fn example_program3() {
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![8]), vec![1]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![7]), vec![0]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![9]), vec![0]);
    }

    #[test]
    fn example_program4() {
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![7]), vec![1]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![8]), vec![0]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![9]), vec![0]);
    }

    #[test]
    fn example_program5() {
        // Here are some jump tests that take an input, then output 0 if the
        // input was zero or 1 if the input was non-zero:
        let prog1 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(IntcodeProg::exec_prog(&prog1, vec![0]),   vec![0]);
        assert_eq!(IntcodeProg::exec_prog(&prog1, vec![100]), vec![1]);
        assert_eq!(IntcodeProg::exec_prog(&prog1, vec![-1]),  vec![1]);

        let prog2 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(IntcodeProg::exec_prog(&prog2, vec![0]),   vec![0]);
        assert_eq!(IntcodeProg::exec_prog(&prog2, vec![100]), vec![1]);
        assert_eq!(IntcodeProg::exec_prog(&prog2, vec![-1]),  vec![1]);
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
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![7]), vec![999]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![8]), vec![1000]);
        assert_eq!(IntcodeProg::exec_prog(&prog, vec![9]), vec![1001]);
    }

    #[test]
    fn example_program7() {
        let prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let output = IntcodeProg::exec_prog(&prog, Vec::new());
        assert_eq!(output, prog);
    }

    #[test]
    fn example_program8() {
        let prog = vec![104, 1125899906842624, 99];
        assert_eq!(
            IntcodeProg::exec_prog(&prog, Vec::new()),
            vec![1125899906842624]
        );
    }
}
