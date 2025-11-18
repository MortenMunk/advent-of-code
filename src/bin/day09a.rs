fn main() {
    let input: Vec<char> = read_input();
    let mut filesys: Vec<String> = init_filesys(&input);
    let mut start: usize = 0;
    let mut end: usize = find_last(&filesys);
    let mut sum: u64 = 0;
    while start < end {
        if filesys[start] == "." {
            while filesys[end] == "." {
                end -= 1;
            }
            if end > start {
                let (left, right) = filesys.split_at_mut(end);
                std::mem::swap(&mut left[start], &mut right[0]);
                end -= 1;
            }
        } else {
            start += 1;
        }
    }
    for (i, item) in filesys.iter().enumerate() {
        if item != "." {
            sum += item.parse::<u64>().unwrap() * i as u64;
        }
    }
    println!("{sum}");
}

fn read_input() -> Vec<char> {
    include_str!("../../inputs/day09.txt").chars().collect()
}

fn init_filesys(input: &[char]) -> Vec<String> {
    let mut filesys: Vec<String> = Vec::new();
    let mut inc: i32 = 0;
    let mut is_free: bool = false;
    input.iter().for_each(|x| {
        if is_free {
            for _ in 0..*x as i32 {
                filesys.push(".".to_string());
            }
            is_free = false;
        } else {
            for _ in 0..*x as i32 {
                filesys.push(inc.to_string());
            }
            inc += 1;
            is_free = true;
        }
    });
    filesys
}

fn find_last(filesys: &[String]) -> usize {
    for (i, item) in filesys.iter().enumerate().rev() {
        if item != "." {
            return i;
        }
    }
    0
}
