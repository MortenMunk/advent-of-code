use std::collections::HashSet;

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

    let mut valid_tiles: HashSet<(usize, usize)> = HashSet::new();
    let red_tile_amount = input.len();
    for i in 0..red_tile_amount {
        let p1 = input[i];
        let p2 = input[(i + 1) % red_tile_amount];
        let (p_start, p_end) = if p1.0 <= p2.0 && p1.1 <= p2.1 {
            (p1, p2)
        } else {
            (p2, p1)
        };

        let min_x = p_start.0.min(p_end.0);
        let max_x = p_start.0.max(p_end.0);
        let min_y = p_start.1.min(p_end.1);
        let max_y = p_start.1.max(p_end.1);

        if min_x == max_x {
            for y in min_y..=max_y {
                valid_tiles.insert((min_x, y));
            }
        } else {
            for x in min_x..=max_x {
                valid_tiles.insert((x, min_y));
            }
        }
    }

    let mut min_x_global = 0;
    let mut max_x_global = 0;
    let mut min_y_global = 0;
    let mut max_y_global = 0;

    // Fill interior lines

    let mut largest = 0;
    for (l, r) in &input {
        for (l_x, r_x) in &input {
            let p1_x = *l;
            let p1_y = *r;
            let p2_x = *l_x;
            let p2_y = *r_x;

            let min_x = p1_x.min(p2_x);
            let max_x = p1_x.max(p2_x);
            let min_y = p1_y.min(p2_y);
            let max_y = p1_y.max(p2_y);

            let mut is_valid = true;

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if !valid_tiles.contains(&(x, y)) {
                        is_valid = false;
                        break;
                    }
                }
                if !is_valid {
                    break;
                }
            }

            if is_valid {
                let diff_l = max_x - min_x + 1;
                let diff_r = max_y - min_y + 1;
                let area = diff_l * diff_r;
                if area > largest {
                    largest = area;
                }
            }
        }
    }

    println!("{}", largest);
}
