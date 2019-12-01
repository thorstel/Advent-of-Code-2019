use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let fuel_sum: u32 = input
        .lines()
        .map(|s| calc_fuel1(s.parse().unwrap()))
        .sum();
    println!("Part 1 = {}", fuel_sum);

    let fuel_sum: i32 = input
        .lines()
        .map(|s| calc_fuel2(s.parse().unwrap()))
        .sum();
    println!("Part 2 = {}", fuel_sum);

    Ok(())
}

fn calc_fuel1(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn calc_fuel2(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel + calc_fuel2(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        assert_eq!(calc_fuel1(12), 2);
        assert_eq!(calc_fuel1(14), 2);
        assert_eq!(calc_fuel1(1969), 654);
        assert_eq!(calc_fuel1(100756), 33583);
    }

    #[test]
    fn example_inputs_part2() {
        assert_eq!(calc_fuel2(14), 2);
        assert_eq!(calc_fuel2(1969), 966);
        assert_eq!(calc_fuel2(100756), 50346);
    }
}
