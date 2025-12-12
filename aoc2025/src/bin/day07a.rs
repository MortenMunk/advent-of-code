fn main() {
    let input: Vec<Vec<char>> = include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    input
        .iter()
        .for_each(|line| {
            for (i, val) in line.iter().enumerate() {
                // func here
            }
        })
        .sum();
}
