fn main() {
    let input: Vec<&str> = include_str!("../../inputs/day03.txt").lines().collect();
    let joltage_sum: u32 = input.iter().map(|x| joltage(x)).sum();
    println!("{}", joltage_sum);
}

fn joltage(str: &str) -> u32 {
    let str: Vec<char> = str.chars().collect();
    let mut tens = 0;
    let mut ones = 0;
    let ref_point = 0;
    for (i, val) in str.iter().enumerate() {
        let val = val.to_digit(10).unwrap();
        if val > tens && i < str.len() {
            tens = val;
        }
    }

    for (i, val) in str.iter().enumerate() {
        let val = val.to_digit(10).unwrap();
        if val > ones && i > ref_point {
            ones = val;
        }
    }
    10 * tens + ones
}
