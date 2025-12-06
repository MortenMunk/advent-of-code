fn main() {
    let input: Vec<Vec<char>> = include_str!("../../inputs/day04.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let mut counter = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, current) in row.iter().enumerate() {
            if *current == '@' && is_movable(i, j, &input) {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}

fn is_movable(row: usize, col: usize, input: &[Vec<char>]) -> bool {
    count_right_adj(row, col, input)
        + count_left_adj(row, col, input)
        + count_top_bottom_adj(row, col, input)
        < 4
}

fn count_right_adj(row: usize, col: usize, input: &[Vec<char>]) -> u8 {
    let mut counter = 0;

    if col + 1 < input[0].len() && input[row][col + 1] == '@' {
        counter += 1;
    }
    if row + 1 < input.len() && col + 1 < input[0].len() && input[row + 1][col + 1] == '@' {
        counter += 1;
    }
    if row > 0 && col + 1 < input[0].len() && input[row - 1][col + 1] == '@' {
        counter += 1;
    }
    counter
}

fn count_left_adj(row: usize, col: usize, input: &[Vec<char>]) -> u8 {
    let mut counter = 0;

    if col > 0 && input[row][col - 1] == '@' {
        counter += 1;
    }
    if row + 1 < input.len() && col > 0 && input[row + 1][col - 1] == '@' {
        counter += 1;
    }
    if row > 0 && col > 0 && input[row - 1][col - 1] == '@' {
        counter += 1;
    }
    counter
}

fn count_top_bottom_adj(row: usize, col: usize, input: &[Vec<char>]) -> u8 {
    let mut counter = 0;

    if row + 1 < input.len() && input[row + 1][col] == '@' {
        counter += 1;
    }
    if row > 0 && input[row - 1][col] == '@' {
        counter += 1;
    }
    counter
}
