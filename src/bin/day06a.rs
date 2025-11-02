fn get_start_pos(matrix: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if matches!(c, '^' | 'v' | '<' | '>') {
                return (y as i32, x as i32);
            }
        }
    }
    (0, 0)
}

fn main() {
    let content: Vec<Vec<char>> = include_str!("../../inputs/day06a.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let start = get_start_pos(&content);
}
