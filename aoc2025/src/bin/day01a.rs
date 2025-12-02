fn main() {
    let input: Vec<Vec<char>> = include_str!("../../inputs/day01.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut current: i32 = 50;
    let mut counter: i32 = 0;
    for elem in input.iter() {
        let dir: char = elem[0];
        let num_move: i32 = elem[1..].iter().collect::<String>().parse::<i32>().unwrap();
        if dir == 'L' {
            current = (current - num_move).rem_euclid(100);
        } else {
            current = (current + num_move).rem_euclid(100);
        }
        if current == 0 {
            counter += 1;
        }
    }
    println!("{}", counter);
}
