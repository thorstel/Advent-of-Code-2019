use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{}", input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        panic!("Not yet implemented!");
    }

    #[test]
    fn example_inputs_part2() {
        panic!("Not yet implemented!");
    }
}
