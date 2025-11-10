#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_start_pos(matrix: &[Vec<char>]) -> Option<(i32, i32, Direction)> {
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let dir = match c {
                '^' => Direction::North,
                '<' => Direction::West,
                '>' => Direction::East,
                'v' => Direction::South,
                _ => continue,
            };
            return Some((y as i32, x as i32, dir));
        }
    }
    None
}

fn main() {
    let content: Vec<Vec<char>> = include_str!("../../inputs/day06a.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    if let Some((y, x, dir)) = get_start_pos(&content) {
        println!("{y}, {x}, {dir:?}")
    } else {
        println!("No position found");
    }
}
