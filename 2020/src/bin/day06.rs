#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

type Input = Vec<Vec<DeclForm>>;
type PartInput = [Vec<DeclForm>];

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    //include_str!("../test_inputs/day06.inp1")
    include_str!("../inputs/day06.inp")
        .split("\n\n")
        .map(|group| group.lines().map(|form| form.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &PartInput) -> u32 {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(DeclForm { bitset: 0 }, |acc, val| acc | *val)
                .count_yes()
        })
        .sum()
}

fn part2(input: &PartInput) -> u32 {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(DeclForm { bitset: !0 }, |acc, val| acc & *val)
                .count_yes()
        })
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct DeclForm {
    bitset: usize,
}

impl DeclForm {
    fn count_yes(&self) -> u32 {
        self.bitset.count_ones()
    }
}

impl BitOr for DeclForm {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bitset: self.bitset | rhs.bitset,
        }
    }
}

impl BitAnd for DeclForm {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bitset: self.bitset & rhs.bitset,
        }
    }
}

impl FromStr for DeclForm {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bitset: usize = 0;
        for c in s.chars().filter(|c| c.is_lowercase()) {
            let pos = c as usize - 'a' as usize;

            let bit = 1 << pos;

            bitset |= bit;
        }

        Ok(DeclForm { bitset })
    }
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
