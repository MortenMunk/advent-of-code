use std::collections::HashMap;

fn main() {
    let input = read_input();
    let mut antennas: HashMap<char, Vec<Cord>> = HashMap::new();
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
}

struct Cord {
    row: i64,
    col: i64,
}

fn read_input() -> Vec<Vec<char>> {
    include_str!("../../inputs/day07a.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
