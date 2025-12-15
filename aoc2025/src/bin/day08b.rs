type Input = Vec<Vec<u64>>;
type DistList = Vec<(u64, usize, usize)>;

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> DSU {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            true
        } else {
            false
        }
    }
}

fn euclid_dist(irow: &[u64], jrow: &[u64]) -> u64 {
    (irow[0] as i64 - jrow[0] as i64).pow(2) as u64
        + (irow[1] as i64 - jrow[1] as i64).pow(2) as u64
        + (irow[2] as i64 - jrow[2] as i64).pow(2) as u64
}

fn init_distances(input: &Input) -> DistList {
    let mut distances: DistList = Vec::new();
    for (i, irow) in input.iter().enumerate() {
        for (j, jrow) in input.iter().enumerate() {
            if i >= j {
                continue;
            }
            let dist = euclid_dist(irow, jrow);
            distances.push((dist, i, j));
        }
    }
    distances.sort_unstable_by_key(|(dist, _, _)| *dist);
    distances
}

fn main() {
    let input: Input = include_str!("../../inputs/day08.txt")
        .lines()
        .map(|x| {
            x.split(",")
                .map(|entry| entry.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let distances = init_distances(&input);
    let mut dsu = DSU::new(input.len());
    let mut connections_made = 0;
    let max_circs = input.len() - 1;
    let mut last_connection: (usize, usize) = (0, 0);

    for (_, i, j) in distances.iter() {
        if connections_made >= max_circs {
            break;
        }

        if dsu.union(*i, *j) {
            connections_made += 1;
            last_connection = (*i, *j);
        }
    }

    let (i, j) = last_connection;
    let x_i = input[i][0] as usize;
    let x_j = input[j][0] as usize;
    let result = x_i * x_j;

    println!("{}", result);
}
