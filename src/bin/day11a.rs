fn main() {
    let mut input = read_input();
    for _ in 0..25 {
        input = blink(input);
        println!("{:?}", input);
    }
}

fn read_input() -> Vec<String> {
    include_str!("../../inputs/day11.txt")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn blink(input: Vec<String>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();
    for x in input {
        if x == "0" {
            new_vec.push("1".to_string());
        } else if x.len() % 2 == 0 {
            let (first, second) = x.split_at(x.len() / 2);
            // remove trailing zeroes
            new_vec.push(first.to_string());
            new_vec.push(second.to_string());
        } else {
            let temp: i64 = x.parse::<i64>().unwrap_or(0) * 2024;
            new_vec.push(temp.to_string());
        }
    }
    new_vec
}
