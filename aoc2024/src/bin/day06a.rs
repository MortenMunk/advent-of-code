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
        loop {
            if content[y as usize][x as usize] != 'X' {
                counter += 1;
                content[y as usize][x as usize] = 'X';
            }

            let (next_y, next_x) = match dir {
                Direction::North => (y - 1, x),
                Direction::East => (y, x + 1),
                Direction::South => (y + 1, x),
                Direction::West => (y, x - 1),
            };

            if next_y < 0 || next_y >= height || next_x < 0 || next_x >= width {
                break;
            }

            if content[next_y as usize][next_x as usize] == '#' {
                dir = change_dir(dir);
            } else {
                y = next_y;
                x = next_x;
            }
        }
        println!("{counter}")
    } else {
        println!("No position found");
    }
}
