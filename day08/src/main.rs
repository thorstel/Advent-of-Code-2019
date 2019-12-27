use std::error::Error;
use std::fs;

const WIDTH:      usize = 25;
const HEIGHT:     usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<_> = fs::read_to_string("input.txt")?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut result   = 0;
    let mut min_zero = usize::max_value();
    let mut image    = [[2u32; WIDTH]; HEIGHT];
    for layer in input.chunks(LAYER_SIZE) {
        let mut num_zero = 0;
        let mut num_one  = 0;
        let mut num_two  = 0;
        for (r, row) in layer.to_vec().chunks(WIDTH).enumerate() {
            for (c, pixel) in row.iter().enumerate() {
                // Part 1
                match pixel {
                    0 => num_zero += 1,
                    1 => num_one  += 1,
                    2 => num_two  += 1,
                    _ => panic!("Invalid pixel!"),
                }
                // Part 2
                if image[r][c] == 2 {
                    image[r][c] = *pixel;
                }
            }
        }
        // Part 1
        if num_zero < min_zero {
            result   = num_one * num_two;
            min_zero = num_zero;
        }
    }
    println!("Part 1 = {}", result);
    println!("Part 2 =");
    for row in image.iter() {
        for tile in row.iter() {
            match tile {
                0 => print!(" "),
                1 => print!("#"),
                _ => panic!("Invalid pixel!"),
            }
        }
        println!();
    }
    Ok(())
}
