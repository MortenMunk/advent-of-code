use std::collections::HashMap;

fn main() {
    let input = read_input();
    let mut antennas: HashMap<char, Vec<Cord>> = HashMap::new();
    let mut counter: i64 = 0;

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, ch) in row.iter().enumerate() {
            if *ch != '.' {
                antennas.entry(*ch).or_default().push(Cord {
                    row: row_index as i64,
                    col: col_index as i64,
                })
            }
        }
    }

    for (key, vals) in antennas {
        for i in 0..vals.len() {
            for j in (i + 1)..vals.len() {
                let p1 = &vals[i];
                let p2 = &vals[j];

                loop {
                    let dr: i64 = p1.row as i64 - p2.row as i64;
                    let dc: i64 = p1.col as i64 - p2.col as i64;

                    if is_in_bounds(, ) {
                        counter += 1;
                    }
                }
            }
        }
    }
}

struct Cord {
    row: i64,
    col: i64,
}

fn is_in_bounds(cord: &Cord, input: &Vec<Vec<char>>) -> bool {
    cord.row >= 0
        && cord.col >= 0
        && (cord.row as usize) < input.len()
        && (cord.col as usize) < input[0].len()
}

fn read_input() -> Vec<Vec<char>> {
    include_str!("../../inputs/day07a.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
