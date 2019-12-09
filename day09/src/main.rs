use std::error::Error;
use std::fs;

use intcode::IntcodeProg;

fn main() -> Result<(), Box<dyn Error>> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let output = IntcodeProg::exec_prog(&prog, vec![1]);
    println!("Part 1 = {}", output.back().unwrap());
    let output = IntcodeProg::exec_prog(&prog, vec![2]);
    println!("Part 2 = {}", output.back().unwrap());
    Ok(())
}
