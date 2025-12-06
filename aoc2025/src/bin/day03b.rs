fn main() {
    let input: Vec<&str> = include_str!("../../inputs/day03.txt").lines().collect();
    let joltage_sum: u64 = input.iter().map(|x| joltage(x)).sum();
    println!("{}", joltage_sum);
}

fn joltage(str: &str) -> u64 {
    let digits: Vec<u32> = str.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let k = 12;
    let mut stack: Vec<u32> = Vec::new();
    let n = digits.len();

    for (i, &d) in digits.iter().enumerate() {
        // remove an element from the stack if we find a valid larger value
        while !stack.is_empty() && *stack.last().unwrap() < d && stack.len() + (n - i - 1) >= k {
            stack.pop();
        }

        // fill the stack if it is less than 12 long
        if stack.len() < k {
            stack.push(d);
        }
    }
    vec_to_num(&stack)
}

fn vec_to_num(queue: &[u32]) -> u64 {
    queue
        .iter()
        .map(|d| char::from_digit(*d, 10).unwrap())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "91199999999991";
        assert_eq!(joltage(input), 999999999991);
    }

    #[test]
    fn test2() {
        let input = "99999999999990";
        assert_eq!(joltage(input), 999999999999);
    }

    #[test]
    fn test3() {
        let input = "99999991111199922229333390";
        assert_eq!(joltage(input), 999999999999);
    }
}
