use std::collections::HashMap;

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

    let mut graph = HashMap::new();
    for line in input {
        graph.insert(line.device, line.output);
    }

    let paths = count_paths("you", &graph);
    println!("{}", paths);
}

fn count_paths(node: &str, graph: &HashMap<&str, Vec<&str>>) -> u64 {
    if node == "out" {
        return 1;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(node) {
        for next in neighbors {
            total += count_paths(next, graph);
        }
    }
    total
}

fn parseline<'a>(s: &'a str) -> LineData<'a> {
    let (device, rest) = s
        .split_once(':')
        .expect("Line must contain device and outputs");
    let output = rest.split_whitespace().collect();

    LineData { device, output }
}
