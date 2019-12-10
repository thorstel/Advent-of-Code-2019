use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let input = match args.next() {
        Some(filename) => fs::read_to_string(filename)?,
        None           => fs::read_to_string("input.txt")?,
    };
    let mut asteroids = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid {
                    x: row as i32,
                    y: col as i32,
                });
            }
        }
    }

    let max_seen = asteroids
        .iter()
        .map(|a| count_seen_asteroids(a, &asteroids))
        .max()
        .unwrap();
    println!("Part 1 = {}", max_seen);
    Ok(())
}

fn count_seen_asteroids(orig: &Asteroid, asteroids: &Vec<Asteroid>) -> usize {
    let mut seen = HashSet::new();
    for asteroid in asteroids {
        if orig != asteroid {
            seen.insert(calc_reference_point(orig, asteroid));
        }
    }
    seen.len()
}

fn calc_reference_point(src: &Asteroid, dst: &Asteroid) -> Asteroid {
    let mut xd = dst.x - src.x;
    let mut yd = dst.y - src.y;
    let gcd = gcd(xd.abs(), yd.abs());
    if gcd != 0 {
        xd /= gcd;
        yd /= gcd;
    }
    Asteroid { x: xd, y: yd }
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}
