use std::error::Error;
use std::fs;

const PATTERN: [i64; 4] = [0, 1, 0, -1];

fn main() -> Result<(), Box<dyn Error>> {
    let input  = fs::read_to_string("input.txt")?;
    let signal = calc_phases(&input, 100);
    println!("Part 1 = {}", signal);
    Ok(())
}

fn calc_phases(input: &str, num_phases: usize) -> String {
    let mut signal: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();
    for _ in 0..num_phases {
        signal = calc_signal(signal);
    }
    let result: Vec<_> = signal.iter().map(|e| e.to_string()).take(8).collect();
    result.concat()
}

fn calc_signal(signal: Vec<i64>) -> Vec<i64> {
    (0..signal.len())
        .map(|pos| calc_elem(&signal, pos).abs() % 10)
        .collect()
}

fn calc_elem(signal: &[i64], out_pos: usize) -> i64 {
    signal
        .iter()
        .enumerate()
        .map(|(pos, elem)| elem * pattern_multiplier(pos, out_pos))
        .sum()
}

fn pattern_multiplier(elem_pos: usize, out_pos: usize) -> i64 {
    let idx = ((elem_pos + 1) / (out_pos + 1)) % PATTERN.len();
    PATTERN[idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        assert_eq!(
            &calc_phases("80871224585914546619083218645595", 100),
            "24176176"
        );
        assert_eq!(
            &calc_phases("19617804207202209144916044189917", 100),
            "73745418"
        );
        assert_eq!(
            &calc_phases("69317163492948606335995924319873", 100),
            "52432133"
        );
    }

    #[test]
    fn example_inputs_part2() {
        panic!("Not yet implemented!");
    }
}
