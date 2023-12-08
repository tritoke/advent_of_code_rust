use anyhow::anyhow;
use std::{mem, str::FromStr};

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<(u32, u32)> {
    let (l1, l2) = include_str!("../../inputs/day06.txt")
        .trim()
        .split_once('\n')
        .expect("Failed to get lines");

    let times = l1
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap);

    let dists = l2
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap);

    times.zip(dists).collect()
}

fn part1(inp: &[(u32, u32)]) -> u32 {
    inp.iter()
        .map(|&(time, record)| (0..time).filter(|t| t * (time - t) > record).count() as u32)
        .product()
}

fn part2(inp: &[(u32, u32)]) -> u32 {
    let (time, record) = inp.iter().fold((0, 0), |(at, ad), (t, d)| {
        (
            at * 10_u64.pow(t.ilog10() + 1) + *t as u64,
            ad * 10_u64.pow(d.ilog10() + 1) + *d as u64,
        )
    });

    (0..time).filter(|t| t * (time - t) > record).count() as u32
}
