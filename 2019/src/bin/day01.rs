#![feature(test)]
extern crate test;

use anyhow::*;

const INPUT_NUM: usize = 0;
type Input = Vec<usize>;
type PartInput = [usize];

fn main() -> Result<()> {
    let input = get_input()?;

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Input> {
    let in_str = [include_str!("../inputs/day01.inp")][INPUT_NUM];

    in_str
        .lines()
        .map(|num| Ok(num.parse::<usize>()?))
        .collect()
}

fn part1(input: &PartInput) -> usize {
    input.iter().map(|mass| mass / 3 + 2).sum()
}

fn part2(input: &PartInput) -> usize {
    input
        .iter()
        .map(|mass| {
            let mut fuel_mass = *mass;
            let mut fuel = 0;
            loop {
                if let Some(fm) = (fuel_mass / 3).checked_sub(2) {
                    fuel_mass = fm;
                    fuel += fm;
                } else {
                    break fuel;
                }
            }
        })
        .sum()
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
