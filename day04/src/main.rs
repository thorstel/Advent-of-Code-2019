fn main() {
    let input = 156218..652527 + 1;
    let num_valid1 = input
        .clone()
        .filter(|n| is_valid1(&n.to_string()))
        .count();
    println!("Part 1 = {}", num_valid1);

    let num_valid2 = input
        .filter(|n| is_valid2(&n.to_string()))
        .count();
    println!("Part 2 = {}", num_valid2);
}

fn is_valid1(pwd: &str) -> bool {
    let     n      = pwd.chars().count();
    let mut pairs1 = pwd.chars().take(n - 1).zip(pwd.chars().skip(1));
    let mut pairs2 = pairs1.clone();
    pairs1.all(|(a, b)| a <= b) && pairs2.any(|(a, b)| a == b)
}

fn is_valid2(pwd: &str) -> bool {
    let mut result = false;
    let mut prev   = '\0';
    let mut streak = 1;
    for c in pwd.chars() {
        if c < prev {
            return false;
        }
        if c == prev {
            streak += 1;
        } else {
            if streak == 2 {
                result = true;
            }
            streak = 1;
        }
        prev = c;
    }
    return result || streak == 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs_part1() {
        assert!(is_valid1("111111"));
        assert!(!is_valid1("223450"));
        assert!(!is_valid1("123789"));
    }

    #[test]
    fn example_inputs_part2() {
        assert!(is_valid2("112233"));
        assert!(!is_valid2("123444"));
        assert!(is_valid2("111122"));
    }
}
