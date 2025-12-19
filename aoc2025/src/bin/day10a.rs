#[derive(Debug)]
struct LineData {
    square: String,
    paren: Vec<Vec<usize>>,
    tail: Vec<usize>,
}

fn main() {
    let input: Vec<LineData> = include_str!("../../inputs/day10.txt")
        .lines()
        .map(parseline)
        .collect();
}

fn parseline(line: &str) -> LineData {
    // Square bracket part
    let pattern_start = line.find('[').unwrap() + 1;
    let pattern_end = line.find(']').unwrap();
    let square_pattern = line[pattern_start..pattern_end].to_string();

    // Squiggly bracket part
    let tail_start = line.find('{').unwrap() + 1;
    let tail_end = line.find('}').unwrap();
    let squiggly_pattern = line[tail_start..tail_end].to_string();
    let squiggly = squiggly_pattern
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    // Parenthesis part
    let middle_part = line[pattern_end + 1..tail_start - 1].to_string();
    let mut groups = Vec::new();
    let mut rest = middle_part;

    while let Some(start) = rest.find('(') {
        let end = rest[start + 1..].find(')').unwrap() + 1;
        let group = &rest[start + 1..end];

        let vals = group.split(',').map(|n| n.parse().unwrap()).collect();

        groups.push(vals);
        rest = rest[end + 1..].to_string();
    }

    LineData {
        square: square_pattern,
        paren: groups,
        tail: squiggly,
    }
}
