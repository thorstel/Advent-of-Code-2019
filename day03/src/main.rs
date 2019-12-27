use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input         = fs::read_to_string("input.txt")?;
    let lines: Vec<_> = input.lines().collect();
    let line1         = extract_line(lines[0]);
    let line2         = extract_line(lines[1]);
    let s1            = line1.keys().collect::<HashSet<&(i32, i32)>>();
    let s2            = line2.keys().collect::<HashSet<&(i32, i32)>>();

    let min_dist = s1
        .intersection(&s2)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|p| manhattan_dist(**p, (0, 0)))
        .min()
        .unwrap();
    println!("Part 1 = {}", min_dist);

    let min_steps = s1
        .intersection(&s2)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|p| line1[p] + line2[p])
        .min()
        .unwrap();
    println!("Part 2 = {}", min_steps);
    Ok(())
}

fn manhattan_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
    (i32::abs(x2 - x1) + i32::abs(y2 - y1)) as u32
}

fn extract_line(line: &str) -> HashMap<(i32, i32), u32> {
    let mut map   = HashMap::new();
    let mut pos   = (0, 0);
    let mut steps = 1;
    for cmd in line.trim().split(',') {
        let dir  = &cmd[0..1];
        let dist = cmd[1..].parse::<i32>().unwrap();
        for d in 1..=dist {
            let new_pos = move_dist(dir, pos, d);
            map.entry(new_pos).or_insert(steps);
            steps += 1;
        }
        pos = move_dist(dir, pos, dist);
    }
    map
}

fn move_dist(dir: &str, point: (i32, i32), dist: i32) -> (i32, i32) {
    let (x, y) = point;
    match dir {
        "R" => (x + dist, y),
        "L" => (x - dist, y),
        "U" => (x, y - dist),
        "D" => (x, y + dist),
        _   => panic!("Invalid command!"),
    }
}
