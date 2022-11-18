#![feature(test)]
extern crate test;

use anyhow::Result;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Vec<i32>, std::num::ParseIntError> {
    [
        include_str!("../inputs/day07.inp"),
        include_str!("../test_inputs/day07.inp1"),
    ][INPUT_NUM]
        .trim()
        .split(',')
        .map(str::parse)
        .collect()
}

fn solve(inp: Vec<i32>) -> (i32, i32) {
    let mut part_1 = i32::MAX;
    let mut part_2 = i32::MAX;

    for pos in 0..=inp.len() as i32 {
        let mut cost1 = 0;
        let mut cost2 = 0;

        for i in inp.iter() {
            let cost = (i - pos).abs();
            cost1 += cost;
            cost2 += (cost * (cost + 1)) / 2;
        }

        part_1 = i32::min(part_1, cost1);
        part_2 = i32::min(part_2, cost2);
    }

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
