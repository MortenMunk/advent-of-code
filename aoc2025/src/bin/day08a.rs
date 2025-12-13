use std::collections::{HashMap, HashSet};

type Input = Vec<Vec<u64>>;

fn main() {
    let input: Input = include_str!("../../inputs/day08.txt")
        .lines()
        .map(|x| {
            x.split(",")
                .map(|entry| entry.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let distances = init_distances(input);
}

fn euclid_dist(irow: &[u64], jrow: &[u64]) -> u64 {
    (irow[0] - jrow[0]).pow(2) + (irow[1] - jrow[1]).pow(2) + (irow[2] - jrow[2]).pow(2)
}

fn init_distances(input: Input) -> HashMap<String, u64> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut distances: HashMap<String, u64> = HashMap::new();
    for (i, irow) in input.iter().enumerate() {
        for (j, jrow) in input.iter().enumerate() {
            if i == j || visited.contains(&j) {
                continue;
            }
            let dist = euclid_dist(irow, jrow);
            distances.insert(format!("{}-{}", i, j), dist);
        }
        visited.insert(i);
    }
    distances
}
