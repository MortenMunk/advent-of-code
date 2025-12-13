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
    const MAX_CONNECTIONS: usize = 1000;

    for (_, i, j) in distances.iter().take(MAX_CONNECTIONS) {
        dsu.union(*i, *j);
    }

    let mut circuit_sizes: Vec<usize> = dsu
        .parent
        .iter()
        .enumerate()
        .filter_map(|(i, p)| if i == *p { Some(dsu.size[i]) } else { None })
        .collect();

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    let size1 = circuit_sizes.get(0).copied().unwrap_or(1);
    let size2 = circuit_sizes.get(1).copied().unwrap_or(1);
    let size3 = circuit_sizes.get(2).copied().unwrap_or(1);
    let result: usize = size1 * size2 * size3;
    println!("{}", result);
}
