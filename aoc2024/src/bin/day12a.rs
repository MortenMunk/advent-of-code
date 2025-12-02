fn main() {}

fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, r: usize, c: usize, plant: char) {
    if visited[r][c] {
        return;
    }
    visited[r][c] = true;

    let h = grid.len();
    let w = grid[0].len();
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dr, dc) in dirs {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr < 0 || nc < 0 || nr >= h as isize || nc >= w as isize {
            continue;
        }

        let nr = nr as usize;
        let nc = nc as usize;

        if !visited[nr][nc] && grid[nr][nc] == plant {
            dfs(grid, visited, nr, nc, plant);
        }
    }
}
