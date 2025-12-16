use std::cmp::{max, min};

fn main() {
    let input: Vec<(usize, usize)> = include_str!("../../inputs/day09.txt")
        .lines()
        .filter_map(|line| {
            line.split_once(',').and_then(|(lhs, rhs)| {
                let lhs = lhs.trim().parse::<usize>().ok()?;
                let rhs = rhs.trim().parse::<usize>().ok()?;
                Some((lhs, rhs))
            })
        })
        .collect();
    let mut largest = 0;
    for (l, r) in &input {
        for (l_x, r_x) in &input {
            let diff_l = max(*l, *l_x) - min(*l, *l_x) + 1;
            let diff_r = max(*r, *r_x) - min(*r, *r_x) + 1;
            let area: usize = diff_l * diff_r;

            if area > largest {
                largest = area;
            }
        }
    }

    println!("{}", largest);
}
