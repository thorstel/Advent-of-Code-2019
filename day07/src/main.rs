use std::cmp;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

use intcode::IntcodeProg;
use intcode::ProgramStatus as IPS;

fn main() -> Result<(), Box<dyn Error>> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut signal         = i64::min_value();
    let mut phase_settings = [0, 1, 2, 3, 4];
    permutohedron::heap_recursive(&mut phase_settings, |permut| {
        signal = cmp::max(signal, amplify_signal(&prog, permut));
    });
    println!("Part 1 = {}", signal);

    let mut signal         = i64::min_value();
    let mut phase_settings = [5, 6, 7, 8, 9];
    permutohedron::heap_recursive(&mut phase_settings, |permut| {
        signal = cmp::max(signal, amplify_signal_feedback(&prog, permut));
    });
    println!("Part 2 = {}", signal);
    Ok(())
}

fn amplify_signal(prog: &[i64], phase_settings: &[i64]) -> i64 {
    let mut out = 0;
    for ps in phase_settings {
        out = *IntcodeProg::exec_prog(prog, vec![*ps, out]).back().unwrap();
    }
    out
}

fn amplify_signal_feedback(prog: &[i64], phase_settings: &[i64]) -> i64 {
    let mut prog0   = IntcodeProg::new(prog);
    let mut prog1   = IntcodeProg::new(prog);
    let mut prog2   = IntcodeProg::new(prog);
    let mut prog3   = IntcodeProg::new(prog);
    let mut prog4   = IntcodeProg::new(prog);
    let mut output0 = VecDeque::new(); // == input of 1 
    let mut output1 = VecDeque::new(); // == input of 2 
    let mut output2 = VecDeque::new(); // == input of 3 
    let mut output3 = VecDeque::new(); // == input of 4 
    let mut output4 = VecDeque::new(); // == input of 0 

    output0.push_back(phase_settings[1]);
    output1.push_back(phase_settings[2]);
    output2.push_back(phase_settings[3]);
    output3.push_back(phase_settings[4]);
    output4.push_back(phase_settings[0]);
    output4.push_back(0);

    loop {
        while prog0.exec_instr(&mut output4, &mut output0) == IPS::Success {}
        while prog1.exec_instr(&mut output0, &mut output1) == IPS::Success {}
        while prog2.exec_instr(&mut output1, &mut output2) == IPS::Success {}
        while prog3.exec_instr(&mut output2, &mut output3) == IPS::Success {}
        loop {
            match prog4.exec_instr(&mut output3, &mut output4) {
                IPS::Success         => (),
                IPS::WaitingForInput => break,
                IPS::Finished        => return *output4.front().unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        let prog = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phase_settings = vec![4, 3, 2, 1, 0];
        assert_eq!(amplify_signal(&prog, &phase_settings), 43210);

        let prog = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phase_settings = vec![0, 1, 2, 3, 4];
        assert_eq!(amplify_signal(&prog, &phase_settings), 54321);

        let prog = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phase_settings = vec![1, 0, 4, 3, 2];
        assert_eq!(amplify_signal(&prog, &phase_settings), 65210);
    }

    #[test]
    fn example_inputs_part2() {
        let prog = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase_settings = vec![9, 8, 7, 6, 5];
        assert_eq!(amplify_signal_feedback(&prog, &phase_settings), 139629729);

        let prog = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phase_settings = vec![9, 7, 8, 5, 6];
        assert_eq!(amplify_signal_feedback(&prog, &phase_settings), 18216);
    }
}
