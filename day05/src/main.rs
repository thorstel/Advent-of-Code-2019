use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let prog: Vec<i32> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let output = intcode::exec_prog(&prog, vec![1]);
    println!("Part 1 = {}", output.back().unwrap());
    let output = intcode::exec_prog(&prog, vec![5]);
    println!("Part 2 = {}", output.back().unwrap());
    Ok(())
}
