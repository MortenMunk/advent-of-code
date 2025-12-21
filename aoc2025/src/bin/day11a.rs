#[derive(Debug)]
struct LineData<'a> {
    device: &'a str,
    output: Vec<&'a str>,
}

fn main() {
    let input: Vec<LineData> = include_str!("../../inputs/day11.txt")
        .lines()
        .map(parseline)
        .collect();
    println!("{:?}", input[0]);
}

fn parseline<'a>(s: &'a str) -> LineData<'a> {
    let (device, rest) = s
        .split_once(':')
        .expect("Line must contain device and outputs");
    let output = rest.split_whitespace().collect();

    LineData { device, output }
}
