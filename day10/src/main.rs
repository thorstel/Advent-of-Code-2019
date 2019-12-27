use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

use num::integer::gcd;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
                    x: col as i32,
                    y: row as i32,
                });
            }
        }
    }

    let (station, mut seen) = asteroids
        .iter()
        .map(|&a| (a, seen_asteroids(a, &asteroids)))
        .max_by(|(_, v), (_, w)| v.len().cmp(&w.len()))
        .unwrap();
    println!("Part 1 = {}", seen.len());

    assert!(seen.len() >= 200, "Part 2 only works with 200+ seen asteroids!");
    seen.sort_by(|&a1, &a2| {
        let q1 = quadrant(station, a1);
        let q2 = quadrant(station, a2);
        if q1 == q2 {
            let d1 = distance(station, a1);
            let d2 = distance(station, a2);
            d1.cmp(&d2)
        } else {
            q1.cmp(&q2)
        }
    });
    let x = seen[199].x;
    let y = seen[199].y;
    println!("Part 2 = {}", (100 * x) + y);
    Ok(())
}

fn seen_asteroids(orig: Asteroid, asteroids: &[Asteroid]) -> Vec<Asteroid> {
    let mut result = Vec::new();
    let mut seen   = HashSet::new();
    for &asteroid in asteroids {
        if orig != asteroid {
            let ref_point = calc_reference_point(orig, asteroid);
            if seen.get(&ref_point) == None {
                seen.insert(ref_point);
                result.push(asteroid);
            }
        }
    }
    result
}

fn calc_reference_point(src: Asteroid, dst: Asteroid) -> Asteroid {
    let mut xd = dst.x - src.x;
    let mut yd = dst.y - src.y;
    let gcd = gcd(xd.abs(), yd.abs());
    if gcd != 0 {
        xd /= gcd;
        yd /= gcd;
    }
    Asteroid { x: xd, y: yd }
}

//  3 | 0 
// ---S---
//  2 | 1 
fn quadrant(station: Asteroid, asteroid: Asteroid) -> usize {
    if      asteroid.x >= station.x && asteroid.y <  station.y { 0 }
    else if asteroid.x >  station.x && asteroid.y >= station.y { 1 }
    else if asteroid.x <= station.x && asteroid.y >  station.y { 2 }
    else                                                       { 3 }
}

fn distance(station: Asteroid, asteroid: Asteroid) -> i32 {
    let dx = asteroid.x - station.x;
    let dy = asteroid.y - station.y;
    if      dx == 0 { 100 * dy        }
    else if dy == 0 { 100 * dx        }
    else            { (100 * dy) / dx }
}
