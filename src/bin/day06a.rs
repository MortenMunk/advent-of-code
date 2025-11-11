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
    let mut counter: i32 = 0;
    let mut content: Vec<Vec<char>> = include_str!("../../inputs/day06a.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    if let Some((mut y, mut x, mut dir)) = get_start_pos(&content) {
        let height: i32 = content.len() as i32;
        let width: i32 = content[0].len() as i32;
        while y >= 0 && y < height && x >= 0 && x < width {
            if content[y as usize][x as usize] != 'X' {
                counter += 1;
                content[y as usize][x as usize] = 'X';
            }
            match dir {
                Direction::North => {
                    if y > 0 && content[(y - 1) as usize][x as usize] != '#' {
                        y -= 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
                Direction::East => {
                    if x + 1 < width && content[y as usize][(x + 1) as usize] != '#' {
                        x += 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
                Direction::South => {
                    if y + 1 < height && content[(y + 1) as usize][x as usize] != '#' {
                        y += 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
                Direction::West => {
                    if x > 0 && content[y as usize][(x - 1) as usize] != '#' {
                        x -= 1;
                    } else {
                        dir = change_dir(dir);
                    }
                }
            }
        }
        println!("{counter}")
    } else {
        println!("No position found");
    }
}
