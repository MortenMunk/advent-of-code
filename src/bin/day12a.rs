fn main() {}

fn dfs_rec(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, s: usize, res: &mut Vec<usize>) {
    visited[s] = true;
    res.push(s);
    for &i in &adj[s] {
        if !visited[i] {
            dfs_rec(adj, visited, i, res);
        }
    }
}

fn dfs(adj: Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; adj.len()];
    let mut res: Vec<usize> = Vec::new();
    dfs_rec(&adj, &mut visited, 0, &mut res);
    res
}
