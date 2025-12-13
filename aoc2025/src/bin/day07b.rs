use std::collections::HashMap;

type Point = (usize, usize);
type Memo = HashMap<Point, usize>;
type Input = Vec<Vec<char>>;

fn main() {
    let input: Input = include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let s_row = 0;
    let s_col = input[0]
        .iter()
        .position(|c| *c == 'S')
        .unwrap_or(input[0].len() / 2);
    let mut memo: Memo = HashMap::new();
    let timelines = solve(s_row, s_col, &input, &mut memo);
    println!("{:?}", timelines);
}

fn solve(row: usize, col: usize, input: &Input, memo: &mut Memo) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let current_pos: Point = (row, col);

    if let Some(result) = memo.get(&current_pos) {
        return *result;
    }

    let mut current_r = row;
    let next_split_or_exit: Option<Point> = loop {
        current_r += 1;
        if current_r >= num_rows {
            break None;
        }

        if col < num_cols && input[current_r][col] == '^' {
            break Some((current_r, col));
        }
    };

    if next_split_or_exit.is_none() {
        memo.insert(current_pos, 1);
        return 1;
    }

    let (splitter_r, splitter_c) = next_split_or_exit.unwrap();
    let next_r = splitter_r + 1;
    let mut timelines = 0;

    // Leftside
    let left_col = splitter_c.saturating_sub(1);
    if left_col < num_cols {
        timelines += solve(next_r, left_col, input, memo);
    }

    // Rightside
    let right_col = splitter_c + 1;
    if right_col < num_cols {
        timelines += solve(next_r, right_col, input, memo);
    }

    memo.insert(current_pos, timelines);
    timelines
}
