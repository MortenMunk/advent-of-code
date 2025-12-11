fn main() {
    let input: Vec<u64> = include_str!("../../inputs/day01.txt")
        .lines()
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let sum: u64 = input.iter().map(|x| (x / 3) - 2).sum();
    println!("{}", sum);
}
