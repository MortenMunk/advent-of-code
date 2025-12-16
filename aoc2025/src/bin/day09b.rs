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
            // Find bouding box
            // Check if valid

            if is_valid_rect(p1, p2, s) {
                let area = diff_l * diff_r;
                if area > largest {
                    largest = area;
                }
            }
        }
    }

    println!("{}", largest);
}
