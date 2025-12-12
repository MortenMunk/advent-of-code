fn main() {
    let input: Vec<Vec<char>> = include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for (li, line) in input.iter().enumerate() {
        for (i, val) in line.iter().enumerate() {
            // solve function DFS
            solve(li, i);
        }
    }
}

fn solve(row: usize, col: usize, input: Vec<Vec<char>>) {
    if row > input.len() {
        //simulate step down
    }
}
