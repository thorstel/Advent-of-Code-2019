use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum Technique {
    DealStack,
    DealIncrement(i128),
    Cut(i128),
}

fn main() -> Result<(), Box<dyn Error>> {
    let     input      = fs::read_to_string("input.txt")?;
    let mut techniques = Vec::new();
    input.lines().for_each(|line| {
        if line == "deal into new stack" {
            techniques.push(Technique::DealStack);
        } else if line.starts_with("deal with increment") {
            let incr: i128 = line[20..].parse().unwrap();
            techniques.push(Technique::DealIncrement(incr));
        } else if line.starts_with("cut") {
            let cut: i128 = line[4..].parse().unwrap();
            techniques.push(Technique::Cut(cut));
        } else {
            panic!("Unknown technique!");
        }
    });

    println!("Part 1 = {}", perform_shuffle1(2019, 10007, &techniques));

    let num_cards:  i128 = 119_315_717_514_047;
    let num_rounds: i128 = 101_741_582_076_661;
    let result = perform_shuffle2(2020, num_cards, num_rounds, &techniques);
    println!("Part 2 = {}", result);
    Ok(())
}

fn perform_shuffle1(mut card: i128, num_cards: i128, techniques: &[Technique]) -> i128 {
    for &technique in techniques {
        card = match technique {
            Technique::DealStack           => num_cards - 1 - card,
            Technique::DealIncrement(incr) => (card * incr) % num_cards,
            Technique::Cut(cut)            => (card - cut + num_cards) % num_cards,
        };
    }
    return card;
}

fn perform_shuffle2(
    pos:        i128,
    num_cards:  i128,
    num_rounds: i128,
    techniques: &[Technique],
) -> i128 {
    let mut a = 1;
    let mut b = 0;
    for &technique in techniques.iter().rev() {
        match technique {
            Technique::DealStack => {
                b += 1;
                b *= -1;
                a *= -1;
            }
            Technique::DealIncrement(incr) => {
                let inv = mod_inv(incr, num_cards);
                a = (a * inv) % num_cards;
                b = (b * inv) % num_cards;
            }
            Technique::Cut(cut) => {
                b += if cut < 0 { cut + num_cards } else { cut };
            }
        }
        a %= num_cards;
        b %= num_cards;
        if a < 0 {
            a += num_cards;
        }
        if b < 0 {
            b += num_cards;
        }
    }
    let res1 = (mod_pow(a, num_rounds, num_cards) * pos) % num_cards;
    let res2 = (b * ((mod_pow(a, num_rounds, num_cards) + num_cards - 1) % num_cards)) % num_cards;
    let res3 = mod_pow(a - 1, num_cards - 2, num_cards);
    (res1 + (res2 * res3)) % num_cards
}

fn mod_inv(a: i128, module: i128) -> i128 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

fn mod_pow(a: i128, exp: i128, module: i128) -> i128 {
    let mut x = 1;
    let mut p = a % module;
    for i in 0..128 {
        if 1 & (exp >> i) == 1 {
            x = x * p % module;
        }
        p = p * p % module;
    }
    x
}
