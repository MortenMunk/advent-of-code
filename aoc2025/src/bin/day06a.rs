fn main() {
    let input: Vec<Vec<&str>> = include_str!("../../inputs/day06.txt")
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();
    for col in 0..input[0].len() - 1 {}
}
