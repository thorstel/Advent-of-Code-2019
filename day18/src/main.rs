use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs;

const MOVES: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let grid: Vec<Vec<char>> = match args.next() {
        Some(filename) => fs::read_to_string(filename)?,
        None           => fs::read_to_string("input.txt")?,
    }
    .lines()
    .map(|l| l.chars().collect())
    .collect();

    let mut startx = 0;
    let mut starty = 0;
    let mut key_target = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                '#' | '.' => (),
                '@' => {
                    startx = x as i32;
                    starty = y as i32;
                }
                _ => {
                    if tile.is_lowercase() {
                        key_target |= 1 << key_to_bit(*tile);
                    }
                }
            }
        }
    }

    println!("Part 1 = {}", bfs_find_keys(&grid, startx, starty, key_target));
    Ok(())
}

fn bfs_find_keys(grid: &Vec<Vec<char>>, xs: i32, ys: i32, target: u32) -> usize {
    let mut queue   = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((xs, ys, 0));
    visited.insert((xs, ys, 0), 0usize);

    while queue.len() > 0 {
        let (x, y, keys) = queue.pop_front().unwrap();
        let steps        = *visited.get(&(x, y, keys)).unwrap() + 1;
        for (xd, yd) in MOVES.iter() {
            let newx = x + *xd;
            let newy = y + *yd;
            if can_move(&grid, newx, newy, keys) {
                let tile         = grid[newy as usize][newx as usize];
                let mut new_keys = keys;
                if tile.is_lowercase() {
                    new_keys |= 1 << key_to_bit(tile);
                }
                if new_keys == target {
                    return steps;
                } else if !visited.contains_key(&(newx, newy, new_keys)) {
                    visited.insert((newx, newy, new_keys), steps);
                    queue.push_back((newx, newy, new_keys));
                }
            }
        }
    }
    unreachable!();
}

fn have_key(mut key: char, keys: u32) -> bool {
    key.make_ascii_lowercase();
    (keys & (1 << key_to_bit(key))) != 0
}

fn key_to_bit(key: char) -> u32 {
    (key as u32) - ('a' as u32)
}

fn can_move(grid: &Vec<Vec<char>>, x: i32, y: i32, keys: u32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;
    if y >= grid.len() || x >= grid[y].len() {
        return false;
    }

    let tile = grid[y][x];
    match tile {
        '#' => false,
        '.' | '@' => true,
        _ => {
            if tile.is_lowercase() {
                true
            } else {
                have_key(tile, keys)
            }
        }
    }
}
