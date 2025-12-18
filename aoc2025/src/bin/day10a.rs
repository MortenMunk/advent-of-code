#[derive(Debug)]
struct LineData {
    square: String,
    paren: Vec<Vec<usize>>,
    tail: Vec<usize>,
}

fn main() {
    let input = include_str!("../../inputs/day10.txt")
        .lines()
        .map(parseline)
        .collect();
}


fn parseline(line) -> LineData {
    // TODO
}
