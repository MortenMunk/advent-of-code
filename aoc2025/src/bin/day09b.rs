use std::cmp::{max, min};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let input: Vec<Point> = include_str!("../../inputs/day09.txt")
        .lines()
        .filter_map(|line| {
            line.split_once(',').and_then(|(lhs, rhs)| {
                Some(Point {
                    x: lhs.trim().parse().ok()?,
                    y: rhs.trim().parse().ok()?,
                })
            })
        })
        .collect();

    let mut largest = 0;
    for &p1 in &input {
        for &p2 in &input {
            if rect_goes_outside(p1, p2, &input) {
                continue;
            }

            let width = (p1.x - p2.x).unsigned_abs() + 1;
            let height = (p1.y - p2.y).unsigned_abs() + 1;
            let area = width * height;

            largest = largest.max(area);
        }
    }

    println!("{}", largest);
}

fn rect_goes_outside(a: Point, b: Point, polygon: &[Point]) -> bool {
    let min_x = min(a.x, b.x);
    let max_x = max(a.x, b.x);
    let min_y = min(a.y, b.y);
    let max_y = max(a.y, b.y);

    let c = Point { x: min_x, y: max_y };
    let d = Point { x: max_x, y: min_y };

    for &p in &[c, d] {
        if !point_on_bound(p, polygon) && !point_inside_polygon(p, polygon) {
            return true;
        }
    }

    for &p in polygon {
        if p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y {
            return true;
        }
    }
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];

        if a.y == b.y {
            let y = a.y;
            let seg_min_x = min(a.x, b.x);
            let seg_max_x = max(a.x, b.x);

            if y > min_y && y < max_y {
                if min_x > seg_min_x && min_x < seg_max_x {
                    return true;
                }
                if max_x > seg_min_x && max_x < seg_max_x {
                    return true;
                }
            }
        }
    }
    false
}

fn point_on_bound(p: Point, polygon: &[Point]) -> bool {
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];

        if (b.x - a.x) * (p.y - a.y) != (b.y - a.y) * (p.x - a.x) {
            continue;
        }

        let dot = (p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y);
        if dot < 0 {
            continue;
        }

        let len_sq = (b.x - a.x).pow(2) + (b.y - a.y).pow(2);
        if dot > len_sq {
            continue;
        }

        return true;
    }
    false
}

fn point_inside_polygon(p: Point, polygon: &[Point]) -> bool {
    let mut inside = false;

    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];

        if a.y == b.y {
            continue;
        }

        let ymin = min(a.y, b.y);
        let ymax = max(a.y, b.y);

        if p.y >= ymin && p.y < ymax && p.x < a.x {
            inside = !inside;
        }
    }

    inside
}
