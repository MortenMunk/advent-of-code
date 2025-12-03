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
        if is_even_invalid(x) {
            counter += x;
            continue;
        }
    }
    counter
}

fn get_digit_count(num: u64) -> usize {
    num.to_string().len()
}

fn is_even_invalid(num: u64) -> bool {
    if !get_digit_count(num).is_multiple_of(2) {
        return false;
    }

    let num = num.to_string();
    let (l, r) = num.split_at(num.len() / 2);
    l == r
}
