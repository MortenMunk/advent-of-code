fn main() {
    let input = include_bytes!("../../inputs/day12.txt");

    let count = input
        .split(|&b| b == b'\n')
        .skip(6 * 5)
        .filter(|line| !line.is_empty())
        .filter(|line| {
            // widht * height
            let colon = line.iter().position(|&b| b == b':').unwrap();
            let (size, rest) = line.split_at(colon);

            let mut dims = size.split(|&b| b == b'x');
            let w = parse_usize(dims.next().unwrap());
            let h = parse_usize(dims.next().unwrap());
            let area = w * h;

            // 9 cost pr area
            let mut required = 0usize;
            for num in rest[1..].split(|&b| b == b' ') {
                required += parse_usize(num) * 9;
            }

            area >= required
        })
        .count();

    println!("{count}");
}

fn parse_usize(bytes: &[u8]) -> usize {
    let mut n = 0usize;
    for &b in bytes {
        n = n * 10 + (b - b'0') as usize;
    }
    n
}
