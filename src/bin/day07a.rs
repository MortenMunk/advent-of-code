fn main() {
    let input = read_input();
    for x in input {
        println!("{x:?}")
    }
}

fn read_input() -> Vec<Vec<i64>> {
    include_str!("../../inputs/day07a.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.replace(":", ""))
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
