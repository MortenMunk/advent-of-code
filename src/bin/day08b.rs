use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input();
    let mut antennas: HashMap<char, Vec<Cord>> = HashMap::new();
    let mut unique = HashSet::new();

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

    for vals in antennas.values() {
        for i in 0..vals.len() {
            for j in (i + 1)..vals.len() {
                let p1 = &vals[i];
                let p2 = &vals[j];

                let dr: i64 = p2.row - p1.row;
                let dc: i64 = p2.col - p1.col;

                let mut dir1 = Cord {
                    row: p1.row - dr,
                    col: p1.col - dc,
                };

                let mut dir2 = Cord {
                    row: p2.row + dr,
                    col: p2.col + dc,
                };

                unique.insert((p1.row, p1.col));
                unique.insert((p2.row, p2.col));

                while is_in_bounds(&dir1, &input) {
                    unique.insert((dir1.row, dir1.col));
                    dir1.row -= dr;
                    dir1.col -= dc;
                }

                while is_in_bounds(&dir2, &input) {
                    unique.insert((dir2.row, dir2.col));
                    dir2.row += dr;
                    dir2.col += dc;
                }
            }
        }
    }
    println!("{}", unique.len());
}

struct Cord {
    row: i64,
    col: i64,
}

fn is_in_bounds(cord: &Cord, input: &[Vec<char>]) -> bool {
    cord.row >= 0
        && cord.col >= 0
        && (cord.row as usize) < input.len()
        && (cord.col as usize) < input[0].len()
}

fn read_input() -> Vec<Vec<char>> {
    include_str!("../../inputs/day08.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
