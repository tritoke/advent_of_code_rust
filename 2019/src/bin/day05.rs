#![feature(test)]
extern crate test;

use anyhow::*;

use aoc::intcode::{IntCodeComputer, MemItem};

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let part_1 = part1(&input)?;
    let part_2 = part2(&input)?;

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<IntCodeComputer> {
    let in_str = [
        include_str!("../inputs/day05.inp"),
        include_str!("../test_inputs/day05.inp1"),
        include_str!("../test_inputs/day05.inp2"),
    ][INPUT_NUM];

    Ok(in_str.parse()?)
}

fn part1(input: &IntCodeComputer) -> Result<MemItem> {
    let mut comp = input.clone();

    comp.input(1);
    comp.run()?;

    comp.consume_output()
        .last()
        .ok_or_else(|| format_err!("Failed to get output - queue empty."))
}

fn part2(input: &IntCodeComputer) -> Result<MemItem> {
    let mut comp = input.clone();

    comp.input(5);
    comp.run()?;

    comp.next_output()
        .ok_or_else(|| format_err!("Failed to get output - queue empty."))
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
