fn main() {
    let input: Vec<char> = read_input();
    let filesys: Vec<String> = init_filesys(&input);
    println!("{filesys:?}");
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
