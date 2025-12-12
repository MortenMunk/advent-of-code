fn main() {
    let input: Vec<Vec<char>> = include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let line_len: usize = input[0].len();
    let mut beam_mask = init_beam_mask(line_len);
    let counter: u64 = input
        .iter()
        .map(|line| {
            let mut split_count: u64 = 0;
            for (i, val) in line.iter().enumerate() {
                if *val == '^' && beam_mask[i] == 1 {
                    split_count += 1;
                    if i > 0 {
                        beam_mask[i - 1] = 1
                    }
                    if i < line_len {
                        beam_mask[i + 1] = 1;
                    }
                    beam_mask[i] = 0;
                }
            }
            split_count
        })
        .sum();
    println!("{}", counter);
}

fn init_beam_mask(input_len: usize) -> Vec<u8> {
    let mut bm = vec![0; input_len];
    bm[input_len / 2] = 1;
    bm
}
