use std::collections::VecDeque;

fn main() {
    let input: Vec<&str> = include_str!("../../inputs/day03.txt").lines().collect();
    let joltage_sum: u64 = input.iter().map(|x| joltage(x)).sum();
    println!("{}", joltage_sum);
}

fn joltage(str: &str) -> u64 {
    let str: Vec<u32> = str.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut queue: VecDeque<u32> = str.iter().take(11).cloned().collect();
    for (i, val) in str.iter().enumerate() {
        if i < str.len() - 11 {
            break;
        }
        if !queue.is_empty() || queue.back() > val {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "1199999999991";
        assert_eq!(joltage(input), 99999999991);
    }

    #[test]
    fn test2() {
        let input = "999999999990";
        assert_eq!(joltage(input), 99999999999);
    }

    #[test]
    fn test3() {
        let input = "9999991111199922229333390";
        assert_eq!(joltage(input), 99999999999);
    }
}
