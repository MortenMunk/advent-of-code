fn main() {
    let input: Vec<&str> = include_str!("../../inputs/day02.txt").split(",").collect();
    let mut counter: u64 = 0;
    for entry in input {
        if let Some((lhs, rhs)) = entry.split_once("-") {
            counter += invalid_ids(lhs, rhs);
        }
    }
    println!("{}", counter);
}

fn invalid_ids(lhs: &str, rhs: &str) -> u64 {
    let lhs = lhs.trim().parse::<u64>().unwrap();
    let rhs = rhs.trim().parse::<u64>().unwrap();

    if rhs < lhs {
        return 0;
    }

    let mut counter: u64 = 0;

    for x in lhs..rhs + 1 {
        if is_invalid(x) {
            counter += x;
            continue;
        }
    }
    counter
}

fn is_invalid(num: u64) -> bool {
    let s = num.to_string();
    let n = s.len();

    for k in 1..=n / 2 {
        if !n.is_multiple_of(k) {
            continue;
        }

        let pattern = &s[0..k];
        let mut repeated = String::new();

        for _ in 0..(n / k) {
            repeated.push_str(pattern);
        }

        if repeated == s {
            return true;
        }
    }
    false
}
