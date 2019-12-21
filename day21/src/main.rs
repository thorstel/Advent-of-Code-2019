use std::error::Error;
use std::fs;

use intcode::IntcodeProg;

fn main() -> Result<(), Box<dyn Error>> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    // Note: This only works when the script files are using unix-style
    // line endings!
    let script1 = include_str!("springscript-pt1")
        .as_bytes()
        .iter()
        .map(|byte| *byte as i64)
        .collect();
    let output = IntcodeProg::exec_prog(&prog, script1);
    println!("Part 1 = {}", output.back().unwrap());

    let script2 = include_str!("springscript-pt2")
        .as_bytes()
        .iter()
        .map(|byte| *byte as i64)
        .collect();
    let output = IntcodeProg::exec_prog(&prog, script2);
    println!("Part 2 = {}", output.back().unwrap());
    Ok(())
}
