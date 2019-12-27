use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

type ChemVec = Vec<(u64, String)>;

const TOTAL_ORE_AMOUNT: u64 = 1_000_000_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let input      = fs::read_to_string("input.txt")?;
    let recipes    = parse_input(&input);
    let ore_amount = calc_ore(1, &recipes);
    println!("Part 1 = {}", ore_amount);

    let fuel_amount = calc_fuel(TOTAL_ORE_AMOUNT / ore_amount, &recipes);
    println!("Part 2 = {}", fuel_amount);
    Ok(())
}

fn calc_fuel(start: u64, recipes: &HashMap<String, (u64, ChemVec)>) -> u64 {
    let mut lower_limit = start;
    let mut upper_limit = lower_limit * 2;
    while lower_limit != upper_limit {
        let fuel = (lower_limit + upper_limit + 1) / 2;
        match calc_ore(fuel, recipes).cmp(&TOTAL_ORE_AMOUNT) {
            Ordering::Less    => lower_limit = fuel,
            Ordering::Greater => upper_limit = (lower_limit + upper_limit) / 2,
            Ordering::Equal   => return fuel,
        }
    }
    lower_limit
}

fn calc_ore(fuel: u64, recipes: &HashMap<String, (u64, ChemVec)>) -> u64 {
    let mut ore_needed = 0;
    let mut leftovers  = HashMap::new();
    let mut needed     = VecDeque::new();
    let (_, chemicals) = &recipes["FUEL"];
    chemicals
        .iter()
        .for_each(|(a, n)| needed.push_back((a * fuel, n)));
    while !needed.is_empty() {
        let (amount, name) = needed.pop_front().unwrap();
        if *name == "ORE" {
            ore_needed += amount;
        } else {
            let have = match leftovers.get(name) {
                Some(n) => *n,
                None    => 0,
            };
            if have < amount {
                let (prod_amount, chemicals) = &recipes[name];
                let mut factor = (amount - have) / *prod_amount;
                if (amount - have) % *prod_amount != 0 {
                    factor += 1;
                }
                let left = ((factor * *prod_amount) + have) - amount;
                leftovers.insert(name, left);
                chemicals
                    .iter()
                    .for_each(|(a, n)| needed.push_back((a * factor, n)));
            } else {
                leftovers.insert(name, have - amount);
            }
        }
    }
    ore_needed
}

fn parse_input(input: &str) -> HashMap<String, (u64, ChemVec)> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let mut it            = line.split("=>");
        let chemicals         = it.next().unwrap();
        let (amount, product) = parse_chemical(it.next().unwrap());
        let mut input_chems   = ChemVec::new();
        chemicals
            .split(',')
            .for_each(|c| input_chems.push(parse_chemical(c)));
        assert_eq!(result.insert(product, (amount, input_chems)), None);
    }
    result
}

fn parse_chemical(tok: &str) -> (u64, String) {
    let mut it = tok.trim().split_whitespace();
    let amount = it.next().unwrap();
    let name   = it.next().unwrap();
    (amount.parse().unwrap(), name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        let input = parse_input(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        );
        assert_eq!(calc_ore(1, &input), 165);

        let input = parse_input(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );
        assert_eq!(calc_ore(1, &input), 13312);

        let input = parse_input(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        );
        assert_eq!(calc_ore(1, &input), 180697);

        let input = parse_input(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        );
        assert_eq!(calc_ore(1, &input), 2210736);
    }

    #[test]
    fn example_inputs_part2() {
        let input = parse_input(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );
        let start = TOTAL_ORE_AMOUNT / calc_ore(1, &input);
        assert_eq!(calc_fuel(start, &input), 82892753);

        let input = parse_input(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        );
        let start = TOTAL_ORE_AMOUNT / calc_ore(1, &input);
        assert_eq!(calc_fuel(start, &input), 5586022);

        let input = parse_input(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        );
        let start = TOTAL_ORE_AMOUNT / calc_ore(1, &input);
        assert_eq!(calc_fuel(start, &input), 460664);
    }
}
