use std::collections::VecDeque;
use std::error::Error;
use std::fs;

use intcode::IntcodeProg;
use intcode::ProgramStatus as IPS;

#[derive(Debug)]
struct Controller {
    prog:   IntcodeProg,
    input:  VecDeque<i64>,
    output: VecDeque<i64>,
}

impl Controller {
    fn exec_until_wait(&mut self) {
        loop {
            match self.prog.exec_instr(&mut self.input, &mut self.output) {
                IPS::Success         => (),
                IPS::WaitingForInput => break,
                IPS::Finished        => unreachable!(),
            }
        }
        self.input.push_back(-1);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut devices = Vec::new();
    for addr in 0..50 {
        let mut dev = Controller {
            prog:   IntcodeProg::new(&prog),
            input:  VecDeque::new(),
            output: VecDeque::new(),
        };
        dev.input.push_back(addr);
        devices.push(dev);
    }

    let mut last_y     = None;
    let mut nat_packet = None;
    let mut p1         = None;
    let mut p2         = None;
    let mut d          = 0;
    while p1 == None || p2 == None {
        devices[d].exec_until_wait();

        // Process sent packets
        while devices[d].output.len() >= 3 {
            let a = devices[d].output.pop_front().unwrap();
            let x = devices[d].output.pop_front().unwrap();
            let y = devices[d].output.pop_front().unwrap();
            if a == 255 {
                if p1 == None {
                    p1 = Some(y);
                }
                nat_packet = Some((x, y));
            } else {
                let addr = a as usize;
                devices[addr].input.push_back(x);
                devices[addr].input.push_back(y);
            }
        }

        // Handle idle state
        let is_idle = devices.iter().all(|dev| dev.input == [-1]);
        if is_idle && nat_packet != None {
            let (x, y) = nat_packet.unwrap();
            devices[0].input.push_back(x);
            devices[0].input.push_back(y);
            if Some(y) == last_y {
                p2 = Some(y);
            }
            last_y = Some(y);
            nat_packet = None;
        }
        d = (d + 1) % devices.len();
    }

    println!("Part 1 = {}", p1.unwrap());
    println!("Part 2 = {}", p2.unwrap());
    Ok(())
}
