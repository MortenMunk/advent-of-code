fn main() {
    let (input, _) = include_str!("../../inputs/day05.txt")
        .split_once("\n\n")
        .expect("cannot split sections");

    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .map(|x| {
            let (lhs, rhs) = x.split_once("-").unwrap();
            let start: u64 = lhs.trim().parse().unwrap();
            let end: u64 = rhs.trim().parse().unwrap();
            (start, end)
        })
        .collect();

    ranges.sort_unstable_by_key(|(s, _)| *s);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_, me)) = merged.last_mut() {
            if start <= *me + 1 {
                // pick whatever end is largest
                *me = (*me).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let mut counter: u64 = 0;
    for (start, end) in merged {
        counter += end - start + 1;
    }

    println!("{}", counter);
}
