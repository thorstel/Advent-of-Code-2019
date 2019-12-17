use std::collections::HashSet;
use std::error::Error;
use std::fs;

use intcode::IntcodeProg;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Part 1 = {}", find_intersections(&input));

    input[0] = 2;
    println!("Part 2 = {}", drive_robot(&input));
    Ok(())
}

fn find_intersections(prog: &[i64]) -> i64 {
    let output = IntcodeProg::exec_prog(&prog, Vec::new());
    let mut intersections = Vec::new();
    let mut scaffolds = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for out in output.iter() {
        match out {
            10 => {
                x  = 0;
                y += 1;
                println!("");
            }
            _ => {
                let c = char::from(*out as u8);
                if c != '.' {
                    scaffolds.insert([x, y]);
                    if     scaffolds.contains(&[x,     y - 1])
                        && scaffolds.contains(&[x - 1, y - 1])
                        && scaffolds.contains(&[x + 1, y - 1])
                        && scaffolds.contains(&[x,     y - 2])
                    {
                        intersections.push([x, y - 1]);
                    }
                }
                x += 1;
                print!("{}", c);
            }
        }
    }
    intersections.iter().map(|pos| pos[0] * pos[1]).sum()
}

fn drive_robot(prog: &[i64]) -> i64 {
    let mut input = Vec::new();

    let m = "A,B,A,B,C,C,B,A,B,C\n";
    let a = "L,12,L,10,R,8,L,12\n";
    let b = "R,8,R,10,R,12\n";
    let c = "L,10,R,12,R,8\n";
    m.as_bytes().iter().for_each(|byte| input.push(*byte as i64));
    a.as_bytes().iter().for_each(|byte| input.push(*byte as i64));
    b.as_bytes().iter().for_each(|byte| input.push(*byte as i64));
    c.as_bytes().iter().for_each(|byte| input.push(*byte as i64));

    // no video feed
    input.push(110);
    input.push(10);

    IntcodeProg::exec_prog(&prog, input).pop_back().unwrap()
}
