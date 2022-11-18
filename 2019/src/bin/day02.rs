#![feature(test, option_result_contains)]
extern crate test;

use anyhow::*;

use aoc::intcode::{IntCodeComputer,MemItem};

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input)?;

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<IntCodeComputer> {
    let in_str = [
        include_str!("../inputs/day02.inp"),
        include_str!("../test_inputs/day02.inp1"),
        include_str!("../test_inputs/day02.inp2"),
        include_str!("../test_inputs/day02.inp3"),
        include_str!("../test_inputs/day02.inp4"),
        include_str!("../test_inputs/day02.inp5"),
    ][INPUT_NUM];

    Ok(in_str.parse()?)
}

fn run(mut computer: IntCodeComputer, noun: MemItem, verb: MemItem) -> Result<MemItem> {
    computer.write(1, noun)?;
    computer.write(2, verb)?;
    computer.run()?;
    computer.read(0).ok_or_else(|| format_err!("Failed to read answer from address 0."))
}

fn solve(input: IntCodeComputer) -> Result<(MemItem, MemItem)> {
    // use the assumed behaviour behavior of the intcode program which
    // evaluates the expression: (noun * a) + verb + b
    // shamelessly stolen from: https://github.com/Voltara/advent2019-fast/blob/master/src/day02.cpp

    let b = run(input.clone(), 0, 0)?;
    let a = run(input, 1, 0)? - b;

	let part1 = (12 * a) + 2 + b;

	let k = 19690720 - b;
	let noun = k / a;
	let verb = k - (noun * a);
	let part2 = 100 * noun + verb;

    Ok((part1, part2))
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| solve(input.clone()))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
