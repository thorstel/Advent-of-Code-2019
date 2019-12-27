use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

const MOVES: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid: Vec<Vec<char>> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let (xs, ys, key_target) = find_start(&grid, &[0, grid[0].len(), 0, grid.len()]);
    println!("Part 1 = {}", bfs_find_keys(&grid, xs as i32, ys as i32, key_target));

    // Prepare map for part 2
    grid[ys - 1][xs - 1] = '@';
    grid[ys - 1][xs    ] = '#';
    grid[ys - 1][xs + 1] = '@';
    grid[ys    ][xs - 1] = '#';
    grid[ys    ][xs    ] = '#';
    grid[ys    ][xs + 1] = '#';
    grid[ys + 1][xs - 1] = '@';
    grid[ys + 1][xs    ] = '#';
    grid[ys + 1][xs + 1] = '@';

    // Solve the 4 quadrants independently
    let steps: usize = vec![
        [0,  xs,            0,  ys        ],
        [xs, grid[0].len(), 0,  ys        ],
        [0,  xs,            ys, grid.len()],
        [xs, grid[0].len(), ys, grid.len()],
    ]
    .iter()
    .map(|borders| {
        let (x, y, target) = find_start(&grid, borders);
        bfs_find_keys(&grid, x as i32, y as i32, target)
    })
    .sum();
    println!("Part 2 = {}", steps);

    Ok(())
}

fn bfs_find_keys(grid: &[Vec<char>], xs: i32, ys: i32, target: u32) -> usize {
    let mut queue   = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((xs, ys, 0));
    visited.insert((xs, ys, 0), 0usize);
    while !queue.is_empty() {
        let (x, y, keys) = queue.pop_front().unwrap();
        let steps = *visited.get(&(x, y, keys)).unwrap() + 1;
        for (xd, yd) in MOVES.iter() {
            let newx = x + *xd;
            let newy = y + *yd;
            if can_move(grid, newx, newy, keys, target) {
                let tile = grid[newy as usize][newx as usize];
                let mut new_keys = keys;
                if tile.is_lowercase() {
                    new_keys |= 1 << key_to_bit(tile);
                }
                if new_keys == target {
                    return steps;
                }
                visited.entry((newx, newy, new_keys)).or_insert_with(|| {
                    queue.push_back((newx, newy, new_keys));
                    steps
                });
            }
        }
    }
    panic!("Failed to obtain all keys!");
}

fn find_start(grid: &[Vec<char>], borders: &[usize; 4]) -> (usize, usize, u32) {
    let mut xs     = 0;
    let mut ys     = 0;
    let mut target = 0;
    for (y, row) in grid.iter().enumerate().take(borders[3]).skip(borders[2]) {
        for (x, &tile) in row.iter().enumerate().take(borders[1]).skip(borders[0]) {
            match tile {
                '#' | '.' => (),
                '@' => {
                    xs = x;
                    ys = y;
                }
                _ => {
                    if tile.is_lowercase() {
                        target |= 1 << key_to_bit(tile);
                    }
                }
            }
        }
    }
    (xs, ys, target)
}

fn contains_key(mut key: char, keys: u32) -> bool {
    key.make_ascii_lowercase();
    (keys & (1 << key_to_bit(key))) != 0
}

fn key_to_bit(key: char) -> u32 {
    (key as u32) - ('a' as u32)
}

fn can_move(grid: &[Vec<char>], x: i32, y: i32, keys: u32, target: u32) -> bool {
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
        '#'       => false,
        '.' | '@' => true,
        _         => tile.is_lowercase()
                  || !contains_key(tile, target)
                  || contains_key(tile, keys),
    }
}
