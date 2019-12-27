use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let grid: Vec<Vec<char>> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    // disgusting setup... 😱
    let mut labels:    HashMap<(usize, usize), String>            = HashMap::new();
    let mut positions: HashMap<String, Vec<(usize, usize, bool)>> = HashMap::new();
    for (r, row) in grid.iter().enumerate().take(grid.len() - 2).skip(2) {
        for (c, &t0) in row.iter().enumerate().take(row.len() - 2) {
            let t1 = grid[r][c + 1];
            let t2 = grid[r][c + 2];
            let is_outer = (c == 0) || ((c + 2) == grid[r].len() - 1);
            if is_label(t0) && is_label(t1) && t2 == '.' {
                let label = t0.to_string() + &t1.to_string();
                labels.insert((r, c + 2), label.clone());
                let pos = positions.entry(label.clone()).or_insert_with(Vec::new);
                pos.push((r, c + 2, is_outer));
            } else if t0 == '.' && is_label(t1) && is_label(t2) {
                let label = t1.to_string() + &t2.to_string();
                labels.insert((r, c), label.clone());
                let pos = positions.entry(label.clone()).or_insert_with(Vec::new);
                pos.push((r, c, is_outer));
            }
        }
    }
    for (r, row) in grid.iter().enumerate().take(grid.len() - 2) {
        for (c, &t0) in row.iter().enumerate().take(row.len() - 2).skip(2) {
            let t1 = grid[r + 1][c];
            let t2 = grid[r + 2][c];
            let is_outer = (r == 0) || ((r + 2) == grid.len() - 1);
            if is_label(t0) && is_label(t1) && t2 == '.' {
                let label = t0.to_string() + &t1.to_string();
                labels.insert((r + 2, c), label.clone());
                let pos = positions.entry(label.clone()).or_insert_with(Vec::new);
                pos.push((r + 2, c, is_outer));
            } else if t0 == '.' && is_label(t1) && is_label(t2) {
                let label = t1.to_string() + &t2.to_string();
                labels.insert((r, c), label.clone());
                let pos = positions.entry(label.clone()).or_insert_with(Vec::new);
                pos.push((r, c, is_outer));
            }
        }
    }

    println!("Part 1 = {}", bfs_find_exit(&grid, &labels, &positions));
    println!("Part 2 = {}", bfs_find_layered_exit(&grid, &labels, &positions));
    Ok(())
}

fn bfs_find_exit(
    grid:      &[Vec<char>],
    labels:    &HashMap<(usize, usize), String>,
    positions: &HashMap<String, Vec<(usize, usize, bool)>>,
) -> usize {
    let (srow, scol, _) = positions.get("AA").unwrap()[0];
    let mut queue       = VecDeque::new();
    let mut visited     = HashMap::new();
    queue.push_back((srow, scol));
    visited.insert((srow, scol), 0);
    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        let steps = *visited.get(&(row, col)).unwrap() + 1;
        for (nrow, ncol, _) in possible_steps(row, col, 1, grid, labels, positions) {
            if let Some(label) = labels.get(&(nrow, ncol)) {
                if label == "ZZ" {
                    return steps;
                }
            }
            visited.entry((nrow, ncol)).or_insert_with(|| {
                queue.push_back((nrow, ncol));
                steps
            });
        }
    }
    panic!("Could not find exit!");
}

fn bfs_find_layered_exit(
    grid:      &[Vec<char>],
    labels:    &HashMap<(usize, usize), String>,
    positions: &HashMap<String, Vec<(usize, usize, bool)>>,
) -> usize {
    let (srow, scol, _) = positions.get("AA").unwrap()[0];
    let mut queue       = VecDeque::new();
    let mut visited     = HashMap::new();
    queue.push_back((srow, scol, 0));
    visited.insert((srow, scol, 0), 0);
    while !queue.is_empty() {
        let (row, col, lvl) = queue.pop_front().unwrap();
        let steps = *visited.get(&(row, col, lvl)).unwrap() + 1;
        for (nrow, ncol, nlvl) in possible_steps(row, col, lvl, grid, labels, positions) {
            if let Some(label) = labels.get(&(nrow, ncol)) {
                if label == "ZZ" && nlvl == 0 {
                    return steps;
                }
            }
            visited.entry((nrow, ncol, nlvl)).or_insert_with(|| {
                queue.push_back((nrow, ncol, nlvl));
                steps
            });
        }
    }
    panic!("Could not find exit!");
}

fn possible_steps(
    row:       usize,
    col:       usize,
    level:     usize,
    grid:      &[Vec<char>],
    labels:    &HashMap<(usize, usize), String>,
    positions: &HashMap<String, Vec<(usize, usize, bool)>>,
) -> Vec<(usize, usize, usize)> {
    let mut steps = Vec::new();
    if grid[row - 1][col] == '.' {
        steps.push((row - 1, col, level));
    }
    if grid[row + 1][col] == '.' {
        steps.push((row + 1, col, level));
    }
    if grid[row][col - 1] == '.' {
        steps.push((row, col - 1, level));
    }
    if grid[row][col + 1] == '.' {
        steps.push((row, col + 1, level));
    }
    if let Some(dst) = get_destination(row, col, level, labels, positions) {
        steps.push(dst);
    }
    steps
}

fn get_destination(
    row:       usize,
    col:       usize,
    level:     usize,
    labels:    &HashMap<(usize, usize), String>,
    positions: &HashMap<String, Vec<(usize, usize, bool)>>,
) -> Option<(usize, usize, usize)> {
    if let Some(label) = labels.get(&(row, col)) {
        let pos = positions.get(label).unwrap();
        if pos.len() != 2 {
            return None;
        }
        let (r0, c0, out0) = pos[0];
        let (r1, c1, out1) = pos[1];
        if (row, col) == (r0, c0) {
            if !out0 {
                return Some((r1, c1, level + 1));
            } else if level > 0 {
                return Some((r1, c1, level - 1));
            }
        } else if !out1 {
            return Some((r0, c0, level + 1));
        } else if level > 0 {
            return Some((r0, c0, level - 1));
        }
    }
    None
}

fn is_label(tile: char) -> bool {
    match tile {
        'A'..='Z' => true,
        _ => false,
    }
}
