fn main() {
    let mut input: Vec<Vec<char>> = include_str!("../../inputs/day04.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let mut change_occured = true;
    let mut counter = 0;
    let rows = input.len();
    let cols = input[0].len();

    while change_occured {
        change_occured = false;
        for i in 0..rows {
            for j in 0..cols {
                if input[i][j] == '@' && is_movable(i, j, &input) {
                    counter += 1;
                    input[i][j] = '-';
                    change_occured = true;
                }
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
