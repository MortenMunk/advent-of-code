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

fn change_dir(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn main() {
    let content: Vec<Vec<char>> = include_str!("../../inputs/day06a.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    if let Some((mut y, mut x, mut dir)) = get_start_pos(&content) {
        let height: i32 = content.len() as i32;
        let width: i32 = content[0].len() as i32;
        let mut counter: i32 = 0;
        while y >= 0 && y < height && x >= 0 && x < width {
            counter += 1;
            match dir {
                Direction::North => {
                    if content[x][y] != '#' {
                        y -= 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
                Direction::East => {
                    if content[x][y] != '#' {
                        x += 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
                Direction::South => {
                    if content[x][y] != '#' {
                        y += 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
                Direction::West => {
                    if content[x][y] != '#' {
                        x -= 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
            }
        }
    } else {
        println!("No position found");
    }
}
