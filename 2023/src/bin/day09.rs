use std::str::FromStr;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<Vec<i64>> {
    include_str!("../../inputs/day09.txt")
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .expect("Failed to parse line")
        })
        .collect()
}

fn yeet(dataset: &[Vec<i64>], accumulator: fn(i64, Vec<i64>) -> i64) -> i64 {
    dataset
        .iter()
        .map(|history| {
            let mut compendium = vec![history.to_owned()];
            while !compendium.last().unwrap().iter().all(|&x| x == 0) {
                let prev = compendium.last().unwrap();
                let next = prev.windows(2).map(|pair| pair[1] - pair[0]).collect();
                _ = prev;
                compendium.push(next);
            }

            compendium.into_iter().rev().fold(0i64, accumulator)
        })
        .sum::<i64>()
}

fn part1(inp: &[Vec<i64>]) -> i64 {
    yeet(inp, |acc, row| acc + row.last().unwrap())
}

fn part2(inp: &[Vec<i64>]) -> i64 {
    yeet(inp, |acc, row| row.first().unwrap() - acc)
}
