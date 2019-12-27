use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<i32> = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut prog = input.clone();
    prog[1] = 12;
    prog[2] = 2;
    println!("Part 1 = {}", exec_program(&mut prog));

    for noun in 0..100 {
        for verb in 0..100 {
            let mut prog = input.clone();
            prog[1] = noun;
            prog[2] = verb;
            if exec_program(&mut prog) == 19_690_720 {
                println!("Part 2 = {}", (100 * noun) + verb);
                return Ok(());
            }
        }
    }
    panic!("Part 2 has failed!");
}

fn exec_program(prog: &mut [i32]) -> i32 {
    let mut pos = 0;
    loop {
        let pos1 = prog[pos + 1] as usize;
        let pos2 = prog[pos + 2] as usize;
        let pos3 = prog[pos + 3] as usize;
        match prog[pos] {
            1  => prog[pos3] = prog[pos1] + prog[pos2],
            2  => prog[pos3] = prog[pos1] * prog[pos2],
            99 => break prog[0],
            _  => panic!("Invalid op code!"),
        }
        pos += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        let mut prog1 = vec![1, 0, 0, 0, 99, 0, 0, 0];
        let res1 = exec_program(&mut prog1);
        assert_eq!(res1, 2);
        assert_eq!(prog1, vec![2, 0, 0, 0, 99, 0, 0, 0]);

        let mut prog2 = vec![2, 3, 0, 3, 99, 0, 0, 0];
        let res2 = exec_program(&mut prog2);
        assert_eq!(res2, 2);
        assert_eq!(prog2, vec![2, 3, 0, 6, 99, 0, 0, 0]);

        let mut prog3 = vec![2, 4, 4, 5, 99, 0, 0, 0];
        let res3 = exec_program(&mut prog3);
        assert_eq!(res3, 2);
        assert_eq!(prog3, vec![2, 4, 4, 5, 99, 9801, 0, 0]);

        let mut prog4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99, 0, 0, 0];
        let res4 = exec_program(&mut prog4);
        assert_eq!(res4, 30);
        assert_eq!(prog4, vec![30, 1, 1, 4, 2, 5, 6, 0, 99, 0, 0, 0]);
    }
}
