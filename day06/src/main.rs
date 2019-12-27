use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let     input  = fs::read_to_string("input.txt")?;
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let obj: Vec<_> = line.trim().split(')').collect();
        orbits.insert(obj[1], obj[0]);
    }

    let dist_center = |start| {
        let mut obj  = start;
        let mut dist = 0;
        loop {
            match orbits.get(obj) {
                Some(x) => {
                    obj = x;
                    dist += 1;
                }
                None => break dist,
            }
        }
    };
    let sum: usize = orbits.keys().map(|k| dist_center(k)).sum();
    println!("Part 1 = {}", sum);

    let path_center = |start| {
        let mut obj  = start;
        let mut path = Vec::new();
        loop {
            match orbits.get(obj) {
                Some(x) => {
                    obj = x;
                    path.push(x);
                }
                None => break path,
            }
        }
    };
    let santa_path = path_center(&"SAN");
    let you_path   = path_center(&"YOU");
    let common     = you_path
        .iter()
        .find(|obj| santa_path.contains(obj))
        .unwrap();
    let you_dist   = you_path.iter().take_while(|obj| *obj != common).count();
    let santa_dist = santa_path.iter().take_while(|obj| *obj != common).count();
    println!("Part 2 = {}", you_dist + santa_dist);
    Ok(())
}
