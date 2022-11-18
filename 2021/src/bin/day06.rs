#![feature(test)]
extern crate test;

use anyhow::Result;
use std::collections::VecDeque;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Vec<u32>, std::num::ParseIntError> {
    [
        include_str!("../inputs/day06.inp"),
        include_str!("../test_inputs/day06.inp1"),
    ][INPUT_NUM]
        .trim()
        .split(',')
        .map(str::parse)
        .collect()
}

fn solve(inp: Vec<u32>) -> (u64, u64) {
    let mut init_counts = [0u64; 9];
    for i in inp {
        init_counts[i as usize] += 1;
    }

    let mut lanternfish = VecDeque::from(init_counts);

    let mut part_1 = 0;

    for day in 1..=256 {
        let birthing = lanternfish.pop_front().unwrap();
        lanternfish.push_back(birthing);
        *lanternfish.get_mut(6).unwrap() += birthing;

        if day == 80 {
            part_1 = lanternfish.iter().sum();
        }
    }

    let part_2 = lanternfish.iter().sum();

    (part_1, part_2)
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| solve(input.clone()))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input().unwrap());
}
