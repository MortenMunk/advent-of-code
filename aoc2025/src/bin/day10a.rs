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
    let pattern_start = line.find('[').unwrap() + 1;
    let pattern_end = line.find(']').unwrap();
    let pattern = line[pattern_start..pattern_end].to_string();
}
