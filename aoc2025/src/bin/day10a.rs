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

    let counter: u64 = input.iter().map(min_presses).sum();
    println!("{}", counter);
}

fn min_presses(linedata: &LineData) -> u64 {
    let mask = init_square_bitmask(&linedata.square);
    let buttons: Vec<u64> = linedata.paren.iter().map(|x| build_mask(x)).collect();

    let mut best = u64::MAX;
    let n = buttons.len();

    for subset in 0..(1u64 << n) {
        let mut state = 0u64;

        (0..n).for_each(|i| {
            if (subset >> i) & 1 == 1 {
                state ^= buttons[i];
            }
        });

        if state == mask {
            let presses = subset.count_ones() as u64;
            best = best.min(presses);
        }
    }
    best
}

fn init_square_bitmask(square: &str) -> u64 {
    let mut mask = 0u64;
    for (i, c) in square.chars().enumerate() {
        if c == '#' {
            mask |= 1 << i;
        }
    }
    mask
}

fn build_mask(button: &[usize]) -> u64 {
    let mut mask = 0u64;
    for x in button {
        mask |= 1 << x;
    }
    mask
}

fn parseline(line: &str) -> LineData {
    // Square bracket part
    let pattern_start = line.find('[').unwrap() + 1;
    let pattern_end = line.find(']').unwrap();
    let square_pattern = line[pattern_start..pattern_end].to_string();

    // Squiggly bracket part
    let tail_start = line.find('{').unwrap() + 1;
    let tail_end = line.find('}').unwrap();
    let squiggly = line[tail_start..tail_end]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    // Parenthesis part
    let mut groups = Vec::new();
    let mut rest = &line[pattern_end + 1..tail_start - 1];

    while let Some(start) = rest.find('(') {
        let end = rest[start + 1..].find(')').unwrap() + start + 1;
        let group = &rest[start + 1..end];

        let vals = group.split(',').map(|n| n.parse().unwrap()).collect();

        groups.push(vals);
        rest = &rest[end + 1..];
    }

    LineData {
        square: square_pattern,
        paren: groups,
        tail: squiggly,
    }
}
