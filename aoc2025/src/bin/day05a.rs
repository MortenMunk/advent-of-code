use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day05.txt");
    let (ranges, ids) = input.split_once("\n\n").expect("cannot split sections");
    let mut counter = 0;

    let mut id_list: HashSet<u64> = HashSet::new();
    ids.lines().for_each(|x| {
        id_list.insert(x.parse::<u64>().unwrap());
    });

    for id in id_list.iter() {
        for x in ranges.lines() {
            if let Some((lhs, rhs)) = x.split_once("-") {
                let start: u64 = lhs.trim().parse::<u64>().unwrap();
                let end: u64 = rhs.trim().parse::<u64>().unwrap();
                if *id >= start && *id <= end {
                    counter += 1;
                    break;
                }
            }
        }
    }
    println!("{}", counter);
}
