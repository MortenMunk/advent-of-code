fn main() {
    let input = read_input();
    let mut sum: i64 = 0;
    for x in input {
        if let Some(x) = calc_loop(x) {
            sum += x;
        }
    }
    println!("{sum:?}")
}

fn try_operation(acc: i64, rest: &[i64], target: i64) -> bool {
    if rest.is_empty() {
        acc == target
    } else {
        let next = rest[0];

        try_operation(acc + next, &rest[1..], target)
            || try_operation(acc * next, &rest[1..], target)
    }
}

fn can_be_calculated(target: i64, arr: &[i64]) -> bool {
    try_operation(arr[0], &arr[1..], target)
}

fn calc_loop(row: Vec<i64>) -> Option<i64> {
    let (rhs, lhs) = row.split_first()?;
    if can_be_calculated(*rhs, lhs) {
        Some(*rhs)
    } else {
        None
    }
}

fn read_input() -> Vec<Vec<i64>> {
    include_str!("../../inputs/day07a.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.replace(":", ""))
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
