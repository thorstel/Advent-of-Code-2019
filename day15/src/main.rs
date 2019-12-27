use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

use intcode::IntcodeProg;
use intcode::ProgramStatus as IPS;

#[derive(Clone, Copy)]
enum MoveCmd {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Clone, Copy)]
enum MoveStatus {
    Wall,
    Move,
    Oxygen,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let prog = IntcodeProg::new(&input);
    let (steps, oxy_prog) = oxygen_bfs(prog);
    println!("Part 1 = {}", steps);
    let (minutes, _) = oxygen_bfs(oxy_prog.unwrap());
    println!("Part 1 = {}", minutes);
    Ok(())
}

fn oxygen_bfs(prog: IntcodeProg) -> (i64, Option<IntcodeProg>) {
    let     moves   = [MoveCmd::North, MoveCmd::South, MoveCmd::West, MoveCmd::East];
    let mut queue   = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(([0, 0], prog));
    visited.insert([0, 0], 0);
    while !queue.is_empty() {
        let (pos, prog) = queue.pop_front().unwrap();
        let steps       = *visited.get(&pos).unwrap();
        for cmd in moves.iter() {
            let new_pos = next_pos(&pos, *cmd);
            if visited.get(&new_pos) == None {
                let mut tmp_prog = prog.clone();
                let     status   = exec_move(&mut tmp_prog, *cmd);
                if status == MoveStatus::Oxygen {
                    // Note: For part 2, Oxygen will not be found again since
                    // it is the first visited position.
                    return (steps + 1, Some(tmp_prog));
                } else if status == MoveStatus::Move {
                    visited.insert(new_pos, steps + 1);
                    queue.push_back((new_pos, tmp_prog));
                }
            }
        }
    }
    (*visited.values().max().unwrap(), None)
}

fn next_pos(pos: &[i64; 2], cmd: MoveCmd) -> [i64; 2] {
    match cmd {
        MoveCmd::North => [pos[0],     pos[1] - 1],
        MoveCmd::South => [pos[0],     pos[1] + 1],
        MoveCmd::West  => [pos[0] - 1, pos[1]    ],
        MoveCmd::East  => [pos[0] + 1, pos[1]    ],
    }
}

fn exec_move(prog: &mut IntcodeProg, cmd: MoveCmd) -> MoveStatus {
    let mut input  = VecDeque::new();
    let mut output = VecDeque::new();
    input.push_back(match cmd {
        MoveCmd::North => 1,
        MoveCmd::South => 2,
        MoveCmd::West  => 3,
        MoveCmd::East  => 4,
    });
    loop {
        match prog.exec_instr(&mut input, &mut output) {
            IPS::Success         => (),
            IPS::WaitingForInput => break,
            IPS::Finished        => panic!("Program should not terminate!"),
        }
    }
    match output.pop_front().unwrap() {
        0 => MoveStatus::Wall,
        1 => MoveStatus::Move,
        2 => MoveStatus::Oxygen,
        _ => panic!("Invalid status!"),
    }
}
