use std::error::Error;
use std::fs;

const PATTERN: [i64; 4] = [0, 1, 0, -1];

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1 = {}", calc_phases(&input));
    println!("Part 2 = {}", calc_phases2(&input));
    Ok(())
}

fn calc_phases(input: &str) -> String {
    let mut signal: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();
    for _ in 0..100 {
        signal = calc_signal(signal);
    }
    let result: Vec<_> = signal.iter().map(|e| e.to_string()).take(8).collect();
    result.concat()
}

fn calc_phases2(input: &str) -> String {
    let offset: usize = input[0..7].parse().unwrap();
    let mut signal: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .cycle()
        .take(input.trim().chars().count() * 10_000)
        .collect();
    for _ in 0..100 {
        signal = calc_signal2(signal);
    }
    let result: Vec<_> = signal
        .iter()
        .skip(offset)
        .map(|e| e.to_string())
        .take(8)
        .collect();
    result.concat()
}

fn calc_signal(signal: Vec<i64>) -> Vec<i64> {
    (0..signal.len())
        .map(|pos| calc_elem(&signal, pos).abs() % 10)
        .collect()
}

fn calc_signal2(signal: Vec<i64>) -> Vec<i64> {
    let mut new_signal = Vec::new();
    let mut sum: i64   = signal.iter().sum();
    for i in 0..signal.len() {
        new_signal.push(sum % 10);
        sum -= signal[i];
    }
    new_signal
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
        assert_eq!(&calc_phases("80871224585914546619083218645595"), "24176176");
        assert_eq!(&calc_phases("19617804207202209144916044189917"), "73745418");
        assert_eq!(&calc_phases("69317163492948606335995924319873"), "52432133");
    }

    #[test]
    fn example_inputs_part2() {
        assert_eq!(&calc_phases2("03036732577212944063491565474664"), "84462026");
        assert_eq!(&calc_phases2("02935109699940807407585447034323"), "78725270");
        assert_eq!(&calc_phases2("03081770884921959731165446850517"), "53553731");
    }
}
