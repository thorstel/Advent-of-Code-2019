use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut prog: Vec<i32> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let output = exec_prog(&mut prog.clone(), vec![1]);
    println!("Part 1 = {}", output.last().unwrap());
    let output = exec_prog(&mut prog, vec![5]);
    println!("Part 2 = {}", output.last().unwrap());
    Ok(())
}

fn exec_prog(prog: &mut [i32], input: Vec<i32>) -> Vec<i32> {
    let mut ip     = 0;
    let mut output = Vec::new();
    let mut input  = input.iter();
    loop {
        let op_code =  prog[ip] %    100;
        let mode1   = (prog[ip] %   1000) /   100 == 1;
        let mode2   = (prog[ip] %  10000) /  1000 == 1;
        let mode3   = (prog[ip] % 100000) / 10000 == 1;

        let get_pos = |mode, pos| {
            if mode {
                pos
            } else {
                prog[pos] as usize
            }
        };

        match op_code {
            1 => { // add
                let val1   = prog[get_pos(mode1, ip + 1)];
                let val2   = prog[get_pos(mode2, ip + 2)];
                let pos    = get_pos(mode3, ip + 3);
                prog[pos]  = val1 + val2;
                ip        += 4;
            }
            2 => { // multiply
                let val1   = prog[get_pos(mode1, ip + 1)];
                let val2   = prog[get_pos(mode2, ip + 2)];
                let pos    = get_pos(mode3, ip + 3);
                prog[pos]  = val1 * val2;
                ip        += 4;
            }
            3 => { // input
                let pos    = prog[ip + 1] as usize;
                prog[pos]  = *input.next().unwrap();
                ip        += 2;
            }
            4 => { // output
                let val = prog[get_pos(mode1, ip + 1)];
                output.push(val);
                ip += 2;
            }
            5 => { // jump-if-true
                let val1 = prog[get_pos(mode1, ip + 1)];
                let val2 = prog[get_pos(mode2, ip + 2)];
                if val1 != 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            }
            6 => { // jump-if-false
                let val1 = prog[get_pos(mode1, ip + 1)];
                let val2 = prog[get_pos(mode2, ip + 2)];
                if val1 == 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            }
            7 => { // less than
                let val1 = prog[get_pos(mode1, ip + 1)];
                let val2 = prog[get_pos(mode2, ip + 2)];
                let pos  = get_pos(mode3, ip + 3);
                if val1 < val2 {
                    prog[pos] = 1;
                } else {
                    prog[pos] = 0;
                }
                ip += 4;
            }
            8 => { // equals
                let val1 = prog[get_pos(mode1, ip + 1)];
                let val2 = prog[get_pos(mode2, ip + 2)];
                let pos  = get_pos(mode3, ip + 3);
                if val1 == val2 {
                    prog[pos] = 1;
                } else {
                    prog[pos] = 0;
                }
                ip += 4;
            }
            99 => break output,
            _  => panic!("Invalid op code!"),
        }
    }
}
