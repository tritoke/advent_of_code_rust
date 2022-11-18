#![feature(test)]
extern crate test;

use anyhow::*;

use aoc::intcode::{IntCodeComputer, MemItem};
use fixedbitset::FixedBitSet;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let part_1 = part1(&input)?;
    //let part_2 = part2(&input)?;

    println!("part 1: {}", part_1);
    //println!("part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<IntCodeComputer> {
    let in_str = [
        include_str!("../inputs/day07.inp"),
        //include_str!("../test_inputs/day05.inp1"),
    ][INPUT_NUM];

    Ok(in_str.parse()?)
}

fn determine_settings(mut computer: IntCodeComputer, amp: MemItem) -> Result<(MemItem, MemItem)> {
    let val = 10000;
    computer.input(amp);
    computer.input(val);
    computer.run()?;
    let value = computer.next_output()
        .ok_or_else(|| format_err!("Failed to get output from computer."))?;

    Ok((value / val, value % val))
}

fn part1(input: &IntCodeComputer) -> Result<MemItem> {
    let coefficients: Vec<_> = (0..=4).map(|amp| determine_settings(input.clone(), amp)).collect::<Result<_>>()?;

    let mut part1: Vec<MemItem> = vec![0; 32];

    for m in 1_usize..32 {
        let bits = FixedBitSet::with_capacity_and_blocks(32, vec![m as u32]);
        for i in bits.ones() {
            let b = 1_usize << i;
            let (mul, add) = coefficients[i];
            let val = mul * part1[m ^ b] + add;
            part1[m] = std::cmp::max(part1[m], val);
        }
    }

    Ok(part1[31])
}

/*
fn part2(input: &IntCodeComputer) -> Result<MemItem> {
    let mut comp = input.clone();

    comp.input(5);
    comp.run()?;

    comp.next_output()
        .ok_or_else(|| format_err!("Failed to get output - queue empty."))
}
*/

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part1(&input))
}

/*
#[bench]
fn bench_part2(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part2(&input))
}
*/

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
