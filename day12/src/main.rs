use std::cmp::Ordering;
use std::ops::AddAssign;

use num::integer::lcm;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Point {
    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Clone)]
struct Moon {
    pos: Point,
    vel: Point,
}

fn main() {
    let moons = vec![
        Moon { pos: Point { x: 17, y:  9, z: -5 }, vel: Point { x: 0, y: 0, z: 0 } },
        Moon { pos: Point { x:  1, y:  7, z: 13 }, vel: Point { x: 0, y: 0, z: 0 } },
        Moon { pos: Point { x: 19, y: 12, z:  5 }, vel: Point { x: 0, y: 0, z: 0 } },
        Moon { pos: Point { x:  6, y: -6, z: -4 }, vel: Point { x: 0, y: 0, z: 0 } },
    ];
    println!("Part 1 = {}", simulate(&moons, 1000));
    println!("Part 2 = {}", find_cycle(&moons));
}

fn simulate(input: &[Moon], steps: usize) -> i64 {
    let mut moons = Vec::from(input);
    for _ in 0..steps {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }
    moons.iter().map(|m| m.pos.energy() * m.vel.energy()).sum()
}

fn find_cycle(input: &[Moon]) -> i64 {
    let mut moons  = Vec::from(input);
    let mut steps  = 0;
    let mut xsteps = 0;
    let mut ysteps = 0;
    let mut zsteps = 0;
    let (xinit, yinit, zinit) = encode_states(&moons);
    while xsteps == 0 || ysteps == 0 || zsteps == 0 {
        steps += 1;
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
        let (xstate, ystate, zstate) = encode_states(&moons);
        if xsteps == 0 && xstate == xinit {
            xsteps = steps;
        }
        if ysteps == 0 && ystate == yinit {
            ysteps = steps;
        }
        if zsteps == 0 && zstate == zinit {
            zsteps = steps;
        }
    }
    lcm(lcm(xsteps, ysteps), zsteps)
}

fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let (dt1, dt2) = gravity_deltas(&moons[i], &moons[j]);
            moons[i].vel += dt1;
            moons[j].vel += dt2;
        }
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        let delta = moons[i].vel;
        moons[i].pos += delta;
    }
}

fn gravity_deltas(moon1: &Moon, moon2: &Moon) -> (Point, Point) {
    let (x1, x2) = match moon1.pos.x.cmp(&moon2.pos.x) {
        Ordering::Less    => ( 1, -1),
        Ordering::Greater => (-1,  1),
        Ordering::Equal   => ( 0,  0),
    };
    let (y1, y2) = match moon1.pos.y.cmp(&moon2.pos.y) {
        Ordering::Less    => ( 1, -1),
        Ordering::Greater => (-1,  1),
        Ordering::Equal   => ( 0,  0),
    };
    let (z1, z2) = match moon1.pos.z.cmp(&moon2.pos.z) {
        Ordering::Less    => ( 1, -1),
        Ordering::Greater => (-1,  1),
        Ordering::Equal   => ( 0,  0),
    };
    (Point { x: x1, y: y1, z: z1 }, Point { x: x2, y: y2, z: z2 })
}

fn encode_states(moons: &[Moon]) -> ([i64; 8], [i64; 8], [i64; 8]) {
    let xstate = [moons[0].pos.x,
                  moons[1].pos.x,
                  moons[2].pos.x,
                  moons[3].pos.x,
                  moons[0].vel.x,
                  moons[1].vel.x,
                  moons[2].vel.x,
                  moons[3].vel.x];
    let ystate = [moons[0].pos.y,
                  moons[1].pos.y,
                  moons[2].pos.y,
                  moons[3].pos.y,
                  moons[0].vel.y,
                  moons[1].vel.y,
                  moons[2].vel.y,
                  moons[3].vel.y];
    let zstate = [moons[0].pos.z,
                  moons[1].pos.z,
                  moons[2].pos.z,
                  moons[3].pos.z,
                  moons[0].vel.z,
                  moons[1].vel.z,
                  moons[2].vel.z,
                  moons[3].vel.z];
    (xstate, ystate, zstate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        let moons = vec![
            Moon { pos: Point { x: -1, y:   0, z:  2 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  2, y: -10, z: -7 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  4, y:  -8, z:  8 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  3, y:   5, z: -1 }, vel: Point { x: 0, y: 0, z: 0 } },
        ];
        assert_eq!(simulate(&moons, 10), 179);

        let moons = vec![
            Moon { pos: Point { x: -8, y: -10, z:  0 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  5, y:   5, z: 10 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  2, y:  -7, z:  3 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  9, y:  -8, z: -3 }, vel: Point { x: 0, y: 0, z: 0 } },
        ];
        assert_eq!(simulate(&moons, 100), 1940);
    }

    #[test]
    fn example_inputs_part2() {
        let moons = vec![
            Moon { pos: Point { x: -1, y:   0, z:  2 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  2, y: -10, z: -7 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  4, y:  -8, z:  8 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  3, y:   5, z: -1 }, vel: Point { x: 0, y: 0, z: 0 } },
        ];
        assert_eq!(find_cycle(&moons), 2772);

        let moons = vec![
            Moon { pos: Point { x: -8, y: -10, z:  0 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  5, y:   5, z: 10 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  2, y:  -7, z:  3 }, vel: Point { x: 0, y: 0, z: 0 } },
            Moon { pos: Point { x:  9, y:  -8, z: -3 }, vel: Point { x: 0, y: 0, z: 0 } },
        ];
        assert_eq!(find_cycle(&moons), 4686774924);
    }
}
