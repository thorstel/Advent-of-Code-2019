use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Vec<usize>> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();

    let mut seen = HashSet::new();
    seen.insert(input.clone());
    let mut grid = input.clone();
    let rating = loop {
        grid = evolve(grid);
        if seen.contains(&grid) {
            break biodiversity_rating(&grid);
        }
        seen.insert(grid.clone());
    };
    println!("Part 1 = {}", rating);

    let mut coords = HashMap::new();
    for (r, row) in input.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if r != 2 || c != 2 {
                coords.insert((0, r, c), tile);
            }
        }
    }
    for _ in 0..200 {
        coords = evolve2(coords);
    }
    let bug_count: usize = coords.values().sum();
    println!("Part 2 = {}", bug_count);
    Ok(())
}

fn biodiversity_rating(grid: &Vec<Vec<usize>>) -> usize {
    let mut power  = 0;
    let mut rating = 0;
    for row in grid.iter() {
        for tile in row.iter() {
            rating += tile * (1 << power);
            power  += 1;
        }
    }
    rating
}

fn evolve(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut new_grid = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        new_grid.push(Vec::new());
        for (c, tile) in row.iter().enumerate() {
            match tile {
                0 => match count_neighbors(r, c, &grid) {
                    1 | 2 => new_grid[r].push(1),
                    _     => new_grid[r].push(0),
                },
                1 => match count_neighbors(r, c, &grid) {
                    1 => new_grid[r].push(1),
                    _ => new_grid[r].push(0),
                },
                _ => panic!("Invalid tile!"),
            }
        }
    }
    new_grid
}

fn evolve2(coords: HashMap<(isize, usize, usize), usize>) -> HashMap<(isize, usize, usize), usize> {
    let mut to_check = HashSet::new();
    for &p in coords.keys() {
        to_check.insert(p);
        for n in get_neighbors(p) {
            to_check.insert(n);
        }
    }

    let mut new_coords = HashMap::new();
    for &p in to_check.iter() {
        let count: usize = get_neighbors(p)
            .iter()
            .map(|p| if let Some(bug) = coords.get(p) { *bug } else { 0 })
            .sum();
        let tile = if coords.contains_key(&p) {
            *coords.get(&p).unwrap()
        } else {
            0
        };
        if tile == 0 {
            match count {
                1 | 2 => new_coords.insert(p, 1),
                _     => new_coords.insert(p, 0),
            };
        } else if tile == 1 {
            match count {
                1 => new_coords.insert(p, 1),
                _ => new_coords.insert(p, 0),
            };
        } else {
            panic!("Invalid tile!");
        }
    }
    new_coords
}

fn count_neighbors(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    if row > 0                   { count += grid[row - 1][col + 0]; }
    if row < grid.len() - 1      { count += grid[row + 1][col + 0]; }
    if col > 0                   { count += grid[row + 0][col - 1]; }
    if col < grid[row].len() - 1 { count += grid[row + 0][col + 1]; }
    count
}

// Did not even try to be clever... 🤯
fn get_neighbors((level, row, col): (isize, usize, usize)) -> Vec<(isize, usize, usize)> {
    let mut neighbors = Vec::new();

    // Up
    if row == 0 {
        neighbors.push((level + 1, 1, 2));
    } else if row == 3 && col == 2 {
        for c in 0..5 {
            neighbors.push((level - 1, 4, c));
        }
    } else {
        neighbors.push((level, row - 1, col));
    }

    // Down
    if row == 4 {
        neighbors.push((level + 1, 3, 2));
    } else if row == 1 && col == 2 {
        for c in 0..5 {
            neighbors.push((level - 1, 0, c));
        }
    } else {
        neighbors.push((level, row + 1, col));
    }

    // Left
    if col == 0 {
        neighbors.push((level + 1, 2, 1));
    } else if row == 2 && col == 3 {
        for r in 0..5 {
            neighbors.push((level - 1, r, 4));
        }
    } else {
        neighbors.push((level, row, col - 1));
    }

    // Right
    if col == 4 {
        neighbors.push((level + 1, 2, 3));
    } else if row == 2 && col == 1 {
        for r in 0..5 {
            neighbors.push((level - 1, r, 0));
        }
    } else {
        neighbors.push((level, row, col + 1));
    }

    neighbors
}
