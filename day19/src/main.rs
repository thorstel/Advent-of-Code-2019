use std::error::Error;
use std::fs;

use intcode::IntcodeProg;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Part 1 = {}", count_beam_points(&input));
    println!("Part 2 = {}", fit_square(&input));
    Ok(())
}

fn count_beam_points(prog: &[i64]) -> i64 {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            count += IntcodeProg::exec_prog(prog, vec![x, y]).back().unwrap();
        }
    }
    count
}

fn fit_square(prog: &[i64]) -> i64 {
    let mut x = 0;
    for y in 100.. {
        while IntcodeProg::exec_prog(prog, vec![x, y]) == [0] {
            x += 1;
        }
        if IntcodeProg::exec_prog(prog, vec![x + 99, y - 99]) == [1] {
            return (x * 10000) + (y - 99);
        }
    }
    unreachable!();
}
