use std::collections::{HashSet, VecDeque};

fn main() {
    let input = read_input();
    let start_points = init_start_points(&input);
    let mut end_points: HashSet<Coord> = HashSet::new();
}

fn search_from(input: &[Vec<char>], start: Coord) -> HashSet<Coord> {
    let mut visited = HashSet::new();
    let mut end_points = HashSet::new();
    let mut frontier = Vec::new().push(start);

    while !frontier.is_empty() {
        let current = frontier.pop;
    }
}

#[derive(Hash, PartialEq, Eq)]
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
