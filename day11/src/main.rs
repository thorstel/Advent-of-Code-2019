use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

use intcode::IntcodeProg;
use intcode::ProgramStatus as IPS;

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn main() -> Result<(), Box<dyn Error>> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let panels = paint_panels(&prog, 0);
    println!("Part 1 = {}", panels.len());

    let panels = paint_panels(&prog, 1);
    let x_iter = panels.keys().map(|(x, _)| x);
    let y_iter = panels.keys().map(|(_, y)| y);
    let x_min  = *x_iter.clone().min().unwrap();
    let x_max  = *x_iter.max().unwrap();
    let y_min  = *y_iter.clone().min().unwrap();
    let y_max  = *y_iter.max().unwrap();

    println!("Part 2 =");
    let mut row = y_min;
    while row <= y_max {
        let mut col = x_min;
        while col <= x_max {
            if match panels.get(&(col, row)) {
                Some(&color) => color,
                None         => 0,
            } == 1 {
                print!("#");
            } else {
                print!(" ");
            }
            col += 1;
        }
        println!();
        row += 1;
    }

    Ok(())
}

fn paint_panels(prog: &[i64], start_color: i64) -> HashMap<(i64, i64), i64> {
    let mut panels = HashMap::new();
    let mut prog   = IntcodeProg::new(prog);
    let mut dir    = Dir::Up;
    let mut xpos   = 0;
    let mut ypos   = 0;
    let mut input  = VecDeque::new();
    let mut output = VecDeque::new();
    input.push_back(start_color);

    loop {
        let result = prog.exec_instr(&mut input, &mut output);
        if output.len() == 2 {
            let color  = output.pop_front().unwrap();
            let turn   = output.pop_front().unwrap();
            panels.insert((xpos, ypos), color);
            dir        = new_dir(dir, turn);
            let (x, y) = new_pos(xpos, ypos, dir);
            xpos       = x;
            ypos       = y;
            let floor_color = match panels.get(&(xpos, ypos)) {
                Some(&color) => color,
                None         => 0,
            };
            input.push_back(floor_color);
        }
        if result == IPS::Finished {
            assert_eq!(output.len(), 0);
            break;
        }
    }
    panels
}

fn new_dir(curr_dir: Dir, turn: i64) -> Dir {
    match turn {
        0 => match curr_dir {
            Dir::Up    => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down  => Dir::Right,
            Dir::Left  => Dir::Down,
        },
        1 => match curr_dir {
            Dir::Up    => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down  => Dir::Left,
            Dir::Left  => Dir::Up,
        },
        _ => panic!("Invalid turn!"),
    }
}

fn new_pos(x: i64, y: i64, dir: Dir) -> (i64, i64) {
    match dir {
        Dir::Up    => (x,     y - 1),
        Dir::Right => (x + 1, y    ),
        Dir::Down  => (x,     y + 1),
        Dir::Left  => (x - 1, y    ),
    }
}
