fn main() {
    let input: Vec<Vec<char>> = include_str!("../../inputs/day01.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let counter: i32 = count_zeros(input);
    println!("{}", counter);
}

fn count_zeros(input: Vec<Vec<char>>) -> i32 {
    let mut current: i32 = 50;
    let mut counter: i32 = 0;
    for elem in input.iter() {
        let dir: char = elem[0];
        let num_move: i32 = elem[1..].iter().collect::<String>().parse::<i32>().unwrap();
        if dir == 'R' {
            let distance_to_zero = 100 - current;
            if num_move >= distance_to_zero {
                counter += (num_move - distance_to_zero) / 100 + 1;
            }
            current = (current + num_move).rem_euclid(100);
        } else {
            if num_move > current + 1 {
                counter += (num_move - current) / 100 + 1;
            }
            current = (current - num_move).rem_euclid(100);
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Vec<Vec<char>> {
        s.lines().map(|l| l.chars().collect()).collect()
    }

    #[test]
    fn test_r_wrap() {
        let input = v("R60");
        assert_eq!(count_zeros(input), 1);
    }

    #[test]
    fn test_triple_r_wrap() {
        let input = v("R300");
        assert_eq!(count_zeros(input), 3);
    }

    #[test]
    fn test_l_wrap() {
        let input = v("L60");
        assert_eq!(count_zeros(input), 1);
    }

    #[test]
    fn test_triple_l_wrap() {
        let input = v("L300");
        assert_eq!(count_zeros(input), 3);
    }

    #[test]
    fn test_triple_l_exact_wrap() {
        let input = v("L250");
        assert_eq!(count_zeros(input), 3);
    }

    #[test]
    fn test_triple_r_exact_wrap() {
        let input = v("R250");
        assert_eq!(count_zeros(input), 3);
    }

    #[test]
    fn test_no_hit() {
        let input1 = v("R30");
        let input2 = v("L30");
        assert_eq!(count_zeros(input1), 0);
        assert_eq!(count_zeros(input2), 0);
    }

    #[test]
    fn test_exact_100_r() {
        let input = v("R100");
        assert_eq!(count_zeros(input), 1);
    }
}
