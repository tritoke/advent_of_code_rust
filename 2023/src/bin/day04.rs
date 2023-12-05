fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<(u128, u128)> {
    include_str!("../../inputs/day04.txt")
        .lines()
        .map(|line| {
            let all_numbers = line.split_once(':').expect("Failed to split on ':'").1;

            let (winners, my_nums) = all_numbers.split_once('|').expect("Failed to split on '|'");
            let mut win = 0;
            let mut mine = 0;

            for num in winners.split_ascii_whitespace() {
                let n: u32 = num.parse().expect("Failed to parse winner");
                win |= 1 << n;
            }

            for num in my_nums.split_ascii_whitespace() {
                let n: u32 = num.parse().expect("Failed to parse my number");
                mine |= 1 << n;
            }

            (win, mine)
        })
        .collect()
}

fn part1(inp: &[(u128, u128)]) -> u32 {
    inp.iter()
        .filter_map(|(win, mine)| (win & mine).count_ones().checked_sub(1).map(|n| 1 << n))
        .sum()
}

fn part2(inp: &[(u128, u128)]) -> u32 {
    inp.iter()
        .enumerate()
        .fold(vec![1; inp.len()], |mut n_copies, (i, (win, mine))| {
            let matches = (win & mine).count_ones() as usize;
            let copies = n_copies[i];
            for card_count in n_copies.iter_mut().skip(i + 1).take(matches) {
                *card_count += copies;
            }
            n_copies
        })
        .into_iter()
        .sum()
}
