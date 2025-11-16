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

    for vals in antennas.values() {
        for i in 0..vals.len() {
            for j in (i + 1)..vals.len() {
                let p1 = &vals[i];
                let p2 = &vals[j];

                let dr: i64 = p1.row - p2.row;
                let dc: i64 = p1.col - p2.col;

                let mut dir1 = Cord {
                    row: p1.row + dr,
                    col: p1.col + dc,
                };

                let mut dir2 = Cord {
                    row: p2.row - dr,
                    col: p2.col - dc,
                };

                while is_in_bounds(&dir1, &input) {
                    counter += 1;
                    dir1.row += dr;
                    dir1.col += dc;
                }

                while is_in_bounds(&dir2, &input) {
                    counter += 1;
                    dir2.row -= dr;
                    dir2.col -= dc;
                }
            }
        }
    }
    println!("{counter}");
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
