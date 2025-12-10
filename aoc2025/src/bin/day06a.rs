fn main() {
    let input: Vec<Vec<&str>> = include_str!("../../inputs/day06.txt")
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();

    let cols = input[0].len();
    let rows = input.len();

    let mut sum: u64 = 0;
    let mut sub_sum: u64 = 0;

    (0..cols).for_each(|col| {
        if input[rows - 1][col] == "+" {
            sub_sum = 0;
            (0..rows - 1).for_each(|row| {
                sub_sum += input[row][col].parse::<u64>().unwrap_or(0);
            });
        } else {
            sub_sum = 1;
            (0..rows - 1).for_each(|row| {
                sub_sum *= input[row][col].parse::<u64>().unwrap_or(0);
            });
        }
        sum += sub_sum;
    });
    println!("{}", sum);
}
