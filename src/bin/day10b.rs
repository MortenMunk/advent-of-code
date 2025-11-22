use std::collections::HashSet;

fn main() {
    let input = read_input();
    let start_points = init_start_points(&input);
    let counter: i64 = start_points.iter().map(|x| search_from(&input, *x)).sum();
    println!("{counter:?}");
}

fn search_from(input: &[Vec<char>], start: Coord) -> i64 {
    fn dfs(input: &[Vec<char>], coord: Coord) -> i64 {
        if input[coord.y][coord.x] == '9' {
            return 1;
        }
        let mut paths: i64 = 0;

        if can_move_up(input, coord) {
            paths += dfs(
                input,
                Coord {
                    x: coord.x,
                    y: coord.y - 1,
                },
            )
        }
        if can_move_down(input, coord) {
            paths += dfs(
                input,
                Coord {
                    x: coord.x,
                    y: coord.y + 1,
                },
            )
        }
        if can_move_left(input, coord) {
            paths += dfs(
                input,
                Coord {
                    x: coord.x - 1,
                    y: coord.y,
                },
            )
        }
        if can_move_right(input, coord) {
            paths += dfs(
                input,
                Coord {
                    x: coord.x + 1,
                    y: coord.y,
                },
            )
        }
        paths
    }
    dfs(input, start)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn read_input() -> Vec<Vec<char>> {
    include_str!("../../inputs/day10.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

fn init_start_points(input: &[Vec<char>]) -> HashSet<Coord> {
    let mut set = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '0' {
                set.insert(Coord { x, y });
            }
        }
    }
    set
}

fn can_move_down(input: &[Vec<char>], coord: Coord) -> bool {
    if coord.y + 1 >= input.len() {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y + 1][coord.x].to_digit(10).unwrap()
    }
}

fn can_move_up(input: &[Vec<char>], coord: Coord) -> bool {
    if coord.y == 0 {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y - 1][coord.x].to_digit(10).unwrap()
    }
}

fn can_move_left(input: &[Vec<char>], coord: Coord) -> bool {
    if coord.x == 0 {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y][coord.x - 1].to_digit(10).unwrap()
    }
}

fn can_move_right(input: &[Vec<char>], coord: Coord) -> bool {
    if coord.x + 1 >= input[0].len() {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y][coord.x + 1].to_digit(10).unwrap()
    }
}
