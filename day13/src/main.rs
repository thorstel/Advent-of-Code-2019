use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

use intcode::IntcodeProg;
use intcode::ProgramStatus as IPS;

fn main() -> Result<(), Box<dyn Error>> {
    let mut prog: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let output = IntcodeProg::exec_prog(&prog, Vec::new());
    let result = Vec::from(output).chunks(3).filter(|c| c[2] == 2).count();
    println!("Part 1 = {}", result);

    prog[0]    = 2;
    let result = play_game(&prog);
    println!("Part 2 = {}", result);
    Ok(())
}

fn play_game(prog: &[i64]) -> i64 {
    let mut prog    = IntcodeProg::new(prog);
    let mut input   = VecDeque::new();
    let mut output  = VecDeque::new();
    let mut blocks  = HashSet::new();
    let mut ballx   = -1;
    let mut paddlex = -1;
    loop {
        let res = prog.exec_instr(&mut input, &mut output);
        if output.len() == 3 {
            let x    = output.pop_front().unwrap();
            let y    = output.pop_front().unwrap();
            let tile = output.pop_front().unwrap();
            if x == -1 && y == 0 && blocks.len() == 0 {
                return tile;
            } else if tile == 0 {
                let _ = blocks.remove(&(x, y));
            } else if tile == 2 {
                let _ = blocks.insert((x, y));
            } else if tile == 3 {
                paddlex = x;
            } else if tile == 4 {
                ballx = x;
            }
        }

        if res == IPS::WaitingForInput {
            input.push_back(match ballx.cmp(&paddlex) {
                Ordering::Less    => -1,
                Ordering::Equal   => 0,
                Ordering::Greater => 1,
            });
        }
    }
}
