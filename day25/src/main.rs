use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::io;

use intcode::IntcodeProg;
use intcode::ProgramStatus as IPS;

fn main() -> Result<(), Box<dyn Error>> {
    let mut prog = IntcodeProg::new(
        &fs::read_to_string("input.txt")?
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>(),
    );
    let mut input  = VecDeque::new();
    let mut output = VecDeque::new();
    loop {
        if !output.is_empty() {
            let c = char::from(output.pop_front().unwrap() as u8);
            print!("{}", c);
        }
        match prog.exec_instr(&mut input, &mut output) {
            IPS::Success         => (),
            IPS::Finished        => break,
            IPS::WaitingForInput => {
                let mut cmd = String::new();
                io::stdin().read_line(&mut cmd).unwrap();
                cmd.trim().chars().for_each(|c| input.push_back(c as i64));
                input.push_back(10);
            }
        }
    }
    Ok(())
}
