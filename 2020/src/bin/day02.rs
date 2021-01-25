#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<(Policy, &'static str)> {
    include_str!("../inputs/day02.inp")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mid = line.find(": ").unwrap();
            let (pol, password) = line.split_at(mid);
            let pass_start = 2;

            let pol_fromstr = pol.parse::<Policy>().unwrap();
            let (_, pass) = password.split_at(pass_start);

            (pol_fromstr, pass)
        })
        .collect()
}

#[derive(Debug)]
struct Policy {
    character: char,
    min: usize,
    max: usize,
}

impl FromStr for Policy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_idx = s.find(' ').unwrap();
        let (range, character) = s.split_at(space_idx);

        let dash_idx = range.find('-').unwrap();
        let (min, _) = range.split_at(dash_idx);
        let (_, max) = range.split_at(dash_idx + 1);

        let min_fromstr = min.parse::<usize>()?;
        let max_fromstr = max.parse::<usize>()?;

        Ok(Policy {
            character: character.chars().nth(1).unwrap(),
            min: min_fromstr,
            max: max_fromstr,
        })
    }
}

impl Policy {
    fn allows(&self, password: &str) -> bool {
        let num_matching = password.chars().filter(|&c| c == self.character).count();

        (self.min <= num_matching) && (self.max >= num_matching)
    }

    fn allows_new(&self, password: &str) -> bool {
        let char1 = password.chars().nth(self.min - 1).unwrap();
        let char2 = password.chars().nth(self.max - 1).unwrap();

        (char1 == self.character) ^ (char2 == self.character)
    }
}

fn part1(input: &Vec<(Policy, &str)>) -> usize {
    input.iter().filter(|(pol, pass)| pol.allows(pass)).count()
}

fn part2(input: &Vec<(Policy, &str)>) -> usize {
    input
        .iter()
        .filter(|(pol, pass)| pol.allows_new(pass))
        .count()
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
