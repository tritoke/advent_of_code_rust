fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<&'static str> {
    include_str!("../../inputs/day01.txt").lines().collect()
}

fn part1(inp: &[&str]) -> u32 {
    inp.iter()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

fn part2(inp: &[&str]) -> u32 {
    inp.iter()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut first = None;
            let mut last = None;
            for (i, c) in bytes.iter().enumerate() {
                let num = if c.is_ascii_digit() {
                    Some((c - b'0') as u32)
                } else {
                    get_number_prefix(&bytes[i..])
                };
                first = first.or(num);
                last = num.or(last);
            }
            first.unwrap() * 10 + last.unwrap()
        })
        .sum()
}

fn get_number_prefix(num: &[u8]) -> Option<u32> {
    #[rustfmt::skip]
    const TABLE: [(&[u8], u32); 9] = [
        (b"one",   1),
        (b"two",   2),
        (b"three", 3),
        (b"four",  4),
        (b"five",  5),
        (b"six",   6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine",  9),
    ];
    TABLE
        .into_iter()
        .find_map(|(prefix, n)| num.starts_with(prefix).then_some(n))
}
