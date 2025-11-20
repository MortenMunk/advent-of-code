fn main() {
    let input = read_input();
}

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

fn can_move_down(input: Vec<Vec<char>>, coord: Coord) -> bool {
    if coord.y >= input.len() {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y + 1][coord.x].to_digit(10).unwrap()
    }
}

fn can_move_up(input: Vec<Vec<char>>, coord: Coord) -> bool {
    if coord.y == 0 {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y - 1][coord.x].to_digit(10).unwrap()
    }
}

fn can_move_left(input: Vec<Vec<char>>, coord: Coord) -> bool {
    if coord.x == 0 {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y][coord.x - 1].to_digit(10).unwrap()
    }
}

fn can_move_right(input: Vec<Vec<char>>, coord: Coord) -> bool {
    if coord.x >= input[0].len() {
        false
    } else {
        input[coord.y][coord.x].to_digit(10).unwrap() + 1
            == input[coord.y][coord.x + 1].to_digit(10).unwrap()
    }
}
